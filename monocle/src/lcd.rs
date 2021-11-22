use std::error::Error;
use std::collections::BTreeSet;
use drm::Device;
use drm::control::Device as ControlDevice;
use drm::control::{self, crtc, framebuffer};
use drm::control::connector::{Interface, State};
use gbm::{BufferObjectFlags, Format};

pub const DISPLAY_WIDTH: usize = 1440; // 1440
pub const DISPLAY_HEIGHT: usize = 2560; // 2560

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
    mut render_callback: impl for<'a> FnMut(&mut drm::control::dumbbuffer::DumbMapping<'a>)
) -> Result<(), Box<dyn Error>> {
    let drm = Card::open("/dev/dri/card0");
    // let drm = gbm::Device::new(card)?;

    drm.acquire_master_lock()?;
    let resource_handles = drm.resource_handles()?;

    let mut chosen_connector = None;
    let mut chosen_mode = None;
    'outer: for _ in 0..100000 {
        for connector in resource_handles.connectors() {
            let info = drm.get_connector(connector.clone())?;
            if info.interface() == Interface::HDMIA {
                if info.state() == State::Connected {
                    let sizes: Vec<(u16, u16)> =
                        info.modes().iter().map(|m| m.size()).collect();
                    if let Some(mode_index) = sizes.iter().position(|s| s == &(DISPLAY_WIDTH as u16, DISPLAY_HEIGHT as u16)) {
                        chosen_connector = Some(connector.clone());
                        chosen_mode = Some(info.modes()[mode_index]);
                        break 'outer;
                    }
                }
            }
        }
    }
    let chosen_connector = chosen_connector.unwrap();
    let chosen_mode = chosen_mode.unwrap();

    let mut chosen_crtc = None;
    'outer: for maybe_encoder in drm.get_connector(chosen_connector.clone())?.encoders() {
        if let Some(encoder) = maybe_encoder {
            let encoder_info = drm.get_encoder(encoder.clone())?;
            for crtc in resource_handles.filter_crtcs(encoder_info.possible_crtcs()) {
                if let None = drm.get_crtc(crtc.clone())?.mode() {
                    chosen_crtc = Some(crtc.clone());
                    break 'outer;
                }
            }
        }
    }
    let chosen_crtc = chosen_crtc.unwrap();

    println!("connector = {:?}", drm.get_connector(chosen_connector.clone())?);
    println!("mode      = {:?}", chosen_mode);
    println!("crtc      = {:?}", drm.get_crtc(chosen_crtc.clone())?);

    for prop in drm.get_properties(chosen_crtc.clone())?.as_props_and_values().0 {
        let info = drm.get_property(prop.clone())?;
        let name = info.name().to_str().unwrap();
        println!("crtc property: name = {}, {:?}", name, info);
        if name == "VRR_ENABLED" {
            drm.set_property(chosen_crtc, prop.clone(),
                             From::from(drm::control::property::Value::Boolean(false)));
        }
    }

    for prop in drm.get_properties(chosen_connector.clone())?.as_props_and_values().0 {
        let info = drm.get_property(prop.clone())?;
        let name = info.name().to_str().unwrap();
        println!("connector property: name = {}, {:?}", name, info);
    }

    for prop in drm.get_properties(chosen_connector.clone())?.as_props_and_values().0 {
        let info = drm.get_property(prop.clone())?;
        let name = info.name().to_str().unwrap();
        println!("connector property: name = {}, {:?}", name, info);
    }

    let mut bo1 = drm.create_dumb_buffer(
        (DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32),
        drm::buffer::DrmFourcc::Argb8888, 32)?;
    let mut bo2 = drm.create_dumb_buffer(
        (DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32),
        drm::buffer::DrmFourcc::Argb8888, 32)?;

    let mut fb1 = drm.add_framebuffer(&bo1, 32, 32)?;
    let mut fb2 = drm.add_framebuffer(&bo2, 32, 32)?;

    let mut bm1 = drm.map_dumb_buffer(&mut bo1)?;
    let mut bm2 = drm.map_dumb_buffer(&mut bo2)?;

    {
        let mut counter: usize = 0;
        for i in 0 .. DISPLAY_WIDTH {
            for _ in 0 .. DISPLAY_HEIGHT {
                bm1.as_mut()[counter + 0] = 0;
                bm1.as_mut()[counter + 1] = 0;
                bm1.as_mut()[counter + 2] = 0;
                bm1.as_mut()[counter + 3] = 0;
                bm2.as_mut()[counter + 0] = 0;
                bm2.as_mut()[counter + 1] = 0;
                bm2.as_mut()[counter + 2] = 0;
                bm2.as_mut()[counter + 3] = 0;
                counter += 4;
            }
        }
    }

    let mut front_dm: &mut drm::control::dumbbuffer::DumbMapping = &mut bm1;
    let mut back_dm: &mut drm::control::dumbbuffer::DumbMapping = &mut bm2;
    let mut front_buffer: &mut drm::control::framebuffer::Handle = &mut fb1;
    let mut back_buffer: &mut drm::control::framebuffer::Handle = &mut fb2;
    drm.set_crtc(chosen_crtc,
                 Some(*front_buffer), (0, 0),
                 &[chosen_connector],
                 Some(chosen_mode))?;
    drm.page_flip(chosen_crtc, *front_buffer, &[drm::control::PageFlipFlags::PageFlipEvent], None);

    loop {
        for event in drm.receive_events()? {
            match event {
                drm::control::Event::PageFlip(page_flip) => {
                    // println!("  Received page flip: frame {:?}, duration {:?}, crtc {:?}",
                    //          page_flip.frame, page_flip.duration, page_flip.crtc);
                    if page_flip.crtc == chosen_crtc {
                        std::mem::swap(&mut front_dm, &mut back_dm);
                        std::mem::swap(&mut front_buffer, &mut back_buffer);
                        drm.page_flip(chosen_crtc, *front_buffer, &[drm::control::PageFlipFlags::PageFlipEvent], None);
                        render_callback(back_dm);
                    }
                },
                _ => {},
            }
        }
    }
}
