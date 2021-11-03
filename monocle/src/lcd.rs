use std::error::Error;
use std::collections::BTreeSet;
use drm::Device;
use drm::control::Device as ControlDevice;
use drm::control::{self, crtc, framebuffer};
use drm::control::connector::{Interface, State};
use drm::control::dumbbuffer::DumbMapping;
use log::*;
//use gbm::{BufferObjectFlags, Format};

pub const RESOLUTIONS: [(usize, usize); 3] = [
    (1440, 2560),
    (2560, 1440),
    (1920, 1080),
];

pub const RESOLUTION_INDEX: usize = 0;

pub const DISPLAY_WIDTH: usize = RESOLUTIONS[RESOLUTION_INDEX].0;
pub const DISPLAY_HEIGHT: usize = RESOLUTIONS[RESOLUTION_INDEX].1;

struct Card(std::fs::File);

impl std::os::unix::io::AsRawFd for Card {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.0.as_raw_fd()
    }
}

impl drm::Device for Card {}

impl drm::control::Device for Card {}

impl Card {
    pub fn open(path: &str) -> Self {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options.write(true);
        Card(options.open(path).unwrap())
    }
}

pub fn run(
    mut render_callback: impl for<'a> FnMut(&mut DumbMapping<'a>) -> bool
) -> Result<(), Box<dyn Error>> {
    let card_device = "/dev/dri/card0";

    debug!("Opening {}", card_device);

    let drm = Card::open(card_device);

    // let drm = gbm::Device::new(card)?;

    debug!("Acquiring libdrm master lock");
    drm.acquire_master_lock()?;

    let resource_handles = drm.resource_handles()?;

    for connector in resource_handles.connectors() {
        let info = drm.get_connector(connector.clone()).unwrap();
        trace!("Connector modes: {:?}", info.modes());
    }

    debug!("Finding appropriate connector and mode");

    let mut chosen_connector: Option<drm::control::connector::Handle> = None;
    let mut chosen_mode: Option<drm::control::Mode> = None;
    'outer1: for _ in 0..100000 {
        for connector in resource_handles.connectors() {
            let info = drm.get_connector(connector.clone())?;
            if info.interface() == Interface::HDMIA {
                if info.state() == State::Connected {
                    let sizes: Vec<(u16, u16)> =
                        info.modes().iter().map(|m| m.size()).collect();
                    if let Some(mode_index) = sizes.iter().position(|s| s == &(DISPLAY_WIDTH as u16, DISPLAY_HEIGHT as u16)) {
                        chosen_connector = Some(connector.clone());
                        chosen_mode = Some(info.modes()[mode_index]);
                        break 'outer1;
                    }
                }
            }
        }
    }
    let chosen_connector = chosen_connector.unwrap();
    let chosen_mode = chosen_mode.unwrap();

    trace!("connector = {:?}", drm.get_connector(chosen_connector.clone())?);
    trace!("mode      = {:?}", chosen_mode);

    debug!("Finding appropriate crtc");
    let mut chosen_crtc = None;
    'outer2: for maybe_encoder in drm.get_connector(chosen_connector.clone())?.encoders() {
        if let Some(encoder) = maybe_encoder {
            let encoder_info = drm.get_encoder(encoder.clone())?;
            for crtc in resource_handles.filter_crtcs(encoder_info.possible_crtcs()) {
                chosen_crtc = Some(crtc.clone());
                break 'outer2;
            }
        }
    }
    let chosen_crtc = chosen_crtc.unwrap();

    trace!("crtc      = {:?}", drm.get_crtc(chosen_crtc.clone())?);

    for prop in drm.get_properties(chosen_crtc.clone())?.as_props_and_values().0 {
        let info = drm.get_property(prop.clone())?;
        let name = info.name().to_str().unwrap();
        debug!("crtc property: name = {}, {:?}", name, info);
        if name == "VRR_ENABLED" {
            drm.set_property(
                chosen_crtc, prop.clone(),
                From::from(drm::control::property::Value::Boolean(false)))?;
        }
    }

    for prop in drm.get_properties(chosen_connector.clone())?.as_props_and_values().0 {
        let info = drm.get_property(prop.clone())?;
        let name = info.name().to_str().unwrap();
        trace!("connector property: name = {}, {:?}", name, info);
    }

    debug!("Creating dumb buffers");

    // drm::buffer::DrmFourcc::Argb8888
    let mut bo1 = drm.create_dumb_buffer(
        (DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32),
        drm::buffer::DrmFourcc::Rgb888, 24)?;
    let mut bo2 = drm.create_dumb_buffer(
        (DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32),
        drm::buffer::DrmFourcc::Rgb888, 24)?;

    debug!("Adding dumb buffers as framebuffers");

    let mut fb1 = drm.add_framebuffer(&bo1, 24, 24)?;
    let mut fb2 = drm.add_framebuffer(&bo2, 24, 24)?;

    debug!("Mapping dumb buffers");

    let mut bm1 = drm.map_dumb_buffer(&mut bo1)?;
    let mut bm2 = drm.map_dumb_buffer(&mut bo2)?;

    debug!("Filling dumb buffers with 0x0");

    bm1.as_mut().fill(0);
    bm2.as_mut().fill(0);

    let mut front_dm: &mut DumbMapping = &mut bm1;
    let mut back_dm: &mut DumbMapping = &mut bm2;
    let mut front_buffer: &mut drm::control::framebuffer::Handle = &mut fb1;
    let mut back_buffer: &mut drm::control::framebuffer::Handle = &mut fb2;

    debug!("Setting the crtc to the front buffer and chosen connector/mode");

    drm.set_crtc(chosen_crtc,
                 Some(*front_buffer), (0, 0),
                 &[chosen_connector],
                 Some(chosen_mode))?;

    debug!("Initial page_flip to pump the event loop");

    drm.page_flip(chosen_crtc,
                  *front_buffer,
                  &[drm::control::PageFlipFlags::PageFlipEvent],
                  None)?;

    debug!("Starting libdrm event loop");

    'event_loop: loop {
        for event in drm.receive_events()? {
            match event {
                drm::control::Event::PageFlip(page_flip) => {
                    // debug!("Received page flip: frame {:?}, duration {:?}, crtc {:?}",
                    //          page_flip.frame, page_flip.duration, page_flip.crtc);
                    if page_flip.crtc == chosen_crtc {
                        std::mem::swap(&mut front_dm, &mut back_dm);
                        std::mem::swap(&mut front_buffer, &mut back_buffer);
                        drm.page_flip(
                            chosen_crtc,
                            *front_buffer,
                            &[drm::control::PageFlipFlags::PageFlipEvent],
                            None)?;
                        if render_callback(back_dm) {
                            debug!("Render callback returned `true`, ending event loop");
                            break 'event_loop;
                        }
                    }
                },
                _ => {},
            }
        }
    }

    debug!("Releasing libdrm master lock");

    drm.release_master_lock()?;

    Ok(())
}
