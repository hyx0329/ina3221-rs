#![doc = include_str!("../README.md")]
#![warn(unsafe_code)]
#![no_std]

mod general;

use embedded_hal::i2c::{Error as I2cError, ErrorKind as I2cErrorKind, I2c};

pub const INA3221_DEFAULT_ADDR: u8 = 0x40;

/// INA3221 error type.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// An I2C error occurred during the transaction.
    I2cError(I2cErrorKind),
    /// Other error. The original error converted from may contain more information.
    Other,
}

impl<T: I2cError> From<T> for Error {
    fn from(value: T) -> Self {
        Self::I2cError(value.kind())
    }
}

impl embedded_hal::digital::Error for Error {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

/// INA3221 struct.
///
/// Shunt resistor values' unit is milli-ohm.
#[derive(Debug)]
pub struct Ina3221<I2C> {
    i2c: I2C,
    address: u8,
    shunt_r1: u8,
    shunt_r2: u8,
    shunt_r3: u8,
}

impl<I2C: I2c> Ina3221<I2C> {
    #[must_use]
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            address: INA3221_DEFAULT_ADDR,
            shunt_r1: 10,
            shunt_r2: 10,
            shunt_r3: 10,
        }
    }

    #[must_use]
    pub fn new_with_addr(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
            shunt_r1: 10,
            shunt_r2: 10,
            shunt_r3: 10,
        }
    }

    #[must_use]
    pub fn shunt_r1(mut self, value: u8) -> Self {
        self.shunt_r1 = value;
        self
    }

    #[must_use]
    pub fn shunt_r2(mut self, value: u8) -> Self {
        self.shunt_r2 = value;
        self
    }

    #[must_use]
    pub fn shunt_r3(mut self, value: u8) -> Self {
        self.shunt_r3 = value;
        self
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }

    pub fn manufacturer_id(&mut self) -> Result<u16, Error> {
        self.read_u16(0xFE)
    }

    pub fn die_id(&mut self) -> Result<u16, Error> {
        self.read_u16(0xFF)
    }

    fn read_u16(&mut self, reg: u8) -> Result<u16, Error> {
        let mut buf: [u8; 2] = [0; 2];
        self.read_buf(reg, &mut buf)?;
        let value: u16 = ((buf[0] as u16) << 8) | (buf[1] as u16);
        Ok(value)
    }

    fn write_u16(&mut self, reg: u8, value: u16) -> Result<(), Error> {
        let mut buf: [u8; 3] = [0; 3];
        buf[0] = reg;
        buf[1] = (value >> 8) as u8;
        buf[2] = (value & 0xF) as u8;
        Ok(self.i2c.write(self.address, &buf)?)
    }

    #[inline]
    fn read_buf(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Error> {
        Ok(self.i2c.write_read(self.address, &[reg], buf)?)
    }
}
