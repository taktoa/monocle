// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright © 2018-2021 Andre Richter <andre.o.richter@gmail.com>
// Copyright © 2021 Google

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};
use std::marker::PhantomData;

pub struct MMIODerefWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<fn() -> T>,
}

impl<T> MMIODerefWrapper<T> {
    /// Create an instance.
    pub unsafe fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for MMIODerefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
    }
}

// GPIO registers.
//
// Descriptions taken from
// - https://datasheets.raspberrypi.org/bcm2711/bcm2711-peripherals.pdf
register_bitfields! {
    u32,

    /// GPIO Function Select 1
    GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART RX

        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART TX
        ]
    ],

    /// GPIO Pin Async. Rising Edge Detect 0
    GPAREN0 [
        AREN0 OFFSET(0) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN1 OFFSET(1) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN2 OFFSET(2) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN3 OFFSET(3) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN4 OFFSET(4) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN5 OFFSET(5) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN6 OFFSET(6) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN7 OFFSET(7) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN8 OFFSET(8) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN9 OFFSET(9) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN10 OFFSET(10) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN11 OFFSET(11) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN12 OFFSET(12) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN13 OFFSET(13) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN14 OFFSET(14) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN15 OFFSET(15) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN16 OFFSET(16) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN17 OFFSET(17) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN18 OFFSET(18) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN19 OFFSET(19) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN20 OFFSET(20) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN21 OFFSET(21) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN22 OFFSET(22) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN23 OFFSET(23) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN24 OFFSET(24) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN25 OFFSET(25) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN26 OFFSET(26) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN27 OFFSET(27) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN28 OFFSET(28) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN29 OFFSET(29) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN30 OFFSET(30) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN31 OFFSET(31) NUMBITS(1) [Off = 0b0, Rising = 0b1],
    ],

    /// GPIO Pin Async. Rising Edge Detect 1
    GPAREN1 [
        AREN32 OFFSET(0) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN33 OFFSET(1) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN34 OFFSET(2) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN35 OFFSET(3) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN36 OFFSET(4) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN37 OFFSET(5) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN38 OFFSET(6) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN39 OFFSET(7) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN40 OFFSET(8) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN41 OFFSET(9) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN42 OFFSET(10) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN43 OFFSET(11) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN44 OFFSET(12) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN45 OFFSET(13) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN46 OFFSET(14) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN47 OFFSET(15) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN48 OFFSET(16) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN49 OFFSET(17) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN50 OFFSET(18) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN51 OFFSET(19) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN52 OFFSET(20) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN53 OFFSET(21) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN54 OFFSET(22) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN55 OFFSET(23) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN56 OFFSET(24) NUMBITS(1) [Off = 0b0, Rising = 0b1],
        AREN57 OFFSET(25) NUMBITS(1) [Off = 0b0, Rising = 0b1],
    ],

    /// GPIO Pull-up / Pull-down Register 0
    ///
    /// BCM2711 only.
    GPIO_PUP_PDN_CNTRL_REG0 [
        /// Pin 15
        GPIO_PUP_PDN_CNTRL15 OFFSET(30) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ],

        /// Pin 14
        GPIO_PUP_PDN_CNTRL14 OFFSET(28) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ]
    ]
}

register_bitfields! {
    u32,
    VADDR [
        VADDR OFFSET(0) NUMBITS(32)
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    PinRegisterBlock {
        (0x00 => _reserved1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _reserved2),
        (0x7c => GPAREN0: ReadWrite<u32, GPAREN0::Register>),
        (0x80 => GPAREN1: ReadWrite<u32, GPAREN1::Register>),
        (0x84 => _reserved3),
        (0xE4 => GPIO_PUP_PDN_CNTRL_REG0: ReadWrite<u32, GPIO_PUP_PDN_CNTRL_REG0::Register>),
        (0xE8 => @END),
    },

    #[allow(non_snake_case)]
    InterruptRegisterBlock {
        (0x00 => _reserved1),
        (0x30 => VADDR: ReadWrite<u32, VADDR::Register>),
        (0x34 => @END),
    }
}

type PinRegisters = MMIODerefWrapper<PinRegisterBlock>;
type InterruptRegisters = MMIODerefWrapper<InterruptRegisterBlock>;

pub struct GPIO {
    pin_registers: PinRegisters,
    ic0_registers: InterruptRegisters,
    ic1_registers: InterruptRegisters,
}

impl GPIO {
    pub unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            pin_registers: PinRegisters::new(mmio_start_addr),
            ic0_registers: InterruptRegisters::new(mmio_start_addr + 0x2000),
            ic1_registers: InterruptRegisters::new(mmio_start_addr + 0x2800),
        }
    }

    pub fn set_rising_interrupt_4(&mut self) {
        self.pin_registers.GPAREN0.write(GPAREN0::AREN4::Rising);
    }

    pub fn get_ic0_vaddr(&self) -> u32 {
        self.ic0_registers.VADDR.read(VADDR::VADDR)
    }

    pub fn get_ic1_vaddr(&self) -> u32 {
        self.ic1_registers.VADDR.read(VADDR::VADDR)
    }
}
