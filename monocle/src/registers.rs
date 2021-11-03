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

    /// GPIO Function Select 0
    GPFSEL0 [
        FSEL0 OFFSET(0) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL1 OFFSET(3) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL2 OFFSET(6) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL3 OFFSET(9) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL4 OFFSET(12) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL5 OFFSET(15) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL6 OFFSET(18) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL7 OFFSET(21) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL8 OFFSET(24) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL9 OFFSET(27) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010]
    ],

    /// GPIO Function Select 1
    GPFSEL1 [
        FSEL10 OFFSET(0) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL11 OFFSET(3) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL12 OFFSET(6) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL13 OFFSET(9) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL14 OFFSET(12) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL15 OFFSET(15) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL16 OFFSET(18) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL17 OFFSET(21) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL18 OFFSET(24) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL19 OFFSET(27) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010]
    ],

    /// GPIO Function Select 2
    GPFSEL2 [
        FSEL20 OFFSET(0) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL21 OFFSET(3) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL22 OFFSET(6) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL23 OFFSET(9) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL24 OFFSET(12) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL25 OFFSET(15) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL26 OFFSET(18) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL27 OFFSET(21) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL28 OFFSET(24) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL29 OFFSET(27) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010]
    ],

    /// GPIO Function Select 3
    GPFSEL3 [
        FSEL30 OFFSET(0) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL31 OFFSET(3) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL32 OFFSET(6) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL33 OFFSET(9) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL34 OFFSET(12) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL35 OFFSET(15) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL36 OFFSET(18) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL37 OFFSET(21) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL38 OFFSET(24) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL39 OFFSET(27) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010]
    ],

    /// GPIO Function Select 4
    GPFSEL4 [
        FSEL40 OFFSET(0) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL41 OFFSET(3) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL42 OFFSET(6) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL43 OFFSET(9) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL44 OFFSET(12) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL45 OFFSET(15) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL46 OFFSET(18) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL47 OFFSET(21) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL48 OFFSET(24) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL49 OFFSET(27) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010]
    ],

    /// GPIO Function Select 5
    GPFSEL5 [
        FSEL50 OFFSET(0) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL51 OFFSET(3) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL52 OFFSET(6) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL53 OFFSET(9) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL54 OFFSET(12) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL55 OFFSET(15) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL56 OFFSET(18) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
        FSEL57 OFFSET(21) NUMBITS(3) [Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010],
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
        (0x00 => GPFSEL0: ReadWrite<u32, GPFSEL0::Register>),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => GPFSEL2: ReadWrite<u32, GPFSEL2::Register>),
        (0x0c => GPFSEL3: ReadWrite<u32, GPFSEL3::Register>),
        (0x10 => GPFSEL4: ReadWrite<u32, GPFSEL4::Register>),
        (0x14 => GPFSEL5: ReadWrite<u32, GPFSEL5::Register>),
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
            pin_registers: PinRegisters::new(mmio_start_addr + 0x200000),
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

    pub fn get_fsel0(&self) -> [u32; 10] {
        [
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL0),
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL1),
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL2),
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL3),
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL4),
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL5),
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL6),
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL7),
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL8),
            self.pin_registers.GPFSEL0.read(GPFSEL0::FSEL9)
       ]
    }

    pub fn get_fsel1(&self) -> [u32; 10] {
        [
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL10),
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL11),
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL12),
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL13),
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL14),
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL15),
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL16),
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL17),
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL18),
            self.pin_registers.GPFSEL1.read(GPFSEL1::FSEL19)
       ]
    }

    pub fn get_fsel2(&self) -> [u32; 10] {
        [
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL20),
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL21),
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL22),
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL23),
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL24),
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL25),
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL26),
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL27),
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL28),
            self.pin_registers.GPFSEL2.read(GPFSEL2::FSEL29)
       ]
    }

    pub fn get_fsel3(&self) -> [u32; 10] {
        [
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL30),
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL31),
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL32),
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL33),
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL34),
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL35),
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL36),
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL37),
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL38),
            self.pin_registers.GPFSEL3.read(GPFSEL3::FSEL39)
       ]
    }

    pub fn get_fsel4(&self) -> [u32; 10] {
        [
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL40),
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL41),
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL42),
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL43),
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL44),
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL45),
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL46),
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL47),
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL48),
            self.pin_registers.GPFSEL4.read(GPFSEL4::FSEL49)
       ]
    }

    pub fn get_fsel5(&self) -> [u32; 8] {
        [
            self.pin_registers.GPFSEL5.read(GPFSEL5::FSEL50),
            self.pin_registers.GPFSEL5.read(GPFSEL5::FSEL51),
            self.pin_registers.GPFSEL5.read(GPFSEL5::FSEL52),
            self.pin_registers.GPFSEL5.read(GPFSEL5::FSEL53),
            self.pin_registers.GPFSEL5.read(GPFSEL5::FSEL54),
            self.pin_registers.GPFSEL5.read(GPFSEL5::FSEL55),
            self.pin_registers.GPFSEL5.read(GPFSEL5::FSEL56),
            self.pin_registers.GPFSEL5.read(GPFSEL5::FSEL57)
       ]
    }
}
