use crate::{Error, I2c, Ina3221};

use num_enum::{FromPrimitive, IntoPrimitive};

#[repr(u8)]
#[derive(IntoPrimitive, FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OperatingMode {
    #[num_enum(default)]
    PowerDown = 0,
    OneshotShunt,
    OneshotBus,
    OneshotShuntBus,
    // This behavior is same with [`OperatingMode::PowerDown`]. Lets ignore it.
    // PowerDownContinuous,
    ContinuousShunt = 5,
    ContinuousBus,
    ContinuousShuntBus,
}

#[repr(u8)]
#[derive(IntoPrimitive, FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AveragingMode {
    #[num_enum(default)]
    Samples1,
    Samples4,
    Samples16,
    Samples64,
    Samples128,
    Samples256,
    Samples512,
    Samples1024,
}

#[repr(u8)]
#[derive(IntoPrimitive, FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConversionTime {
    #[num_enum(default)]
    T140us,
    T204us,
    T332us,
    T588us,
    T1100us,
    T2116us,
    T4156us,
    T8244us,
}

impl<I2C: I2c> Ina3221<I2C> {
    /// Resets the chip, equivalent to power cycling the chip.
    /// 
    /// All registers will be set to default state.
    pub fn reset(&mut self) -> Result<(), Error> {
        self.write_u16(0x00, 0x8000)
    }

    /// gets current power mode
    pub fn power_mode(&mut self) -> Result<OperatingMode, Error> {
        let mode = (self.read_u16(0x00)? & 0b111) as u8;
        Ok(OperatingMode::from_primitive(mode))
    }

    /// sets power mode
    pub fn set_power_mode(&mut self, value: OperatingMode) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_value: u16 = value as u8 as u16;
        let new_state = original_state & 0xFFFC | new_value;
        self.write_u16(0x00, new_state)
    }

    /// gets value averaging mode
    pub fn averaging_mode(&mut self) -> Result<AveragingMode, Error> {
        let mode = (self.read_u16(0x00)? >> 9 & 0b111) as u8;
        Ok(AveragingMode::from_primitive(mode))
    }

    /// sets value averaging mode
    pub fn set_averaging_mode(&mut self, value: AveragingMode) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_value: u16 = value as u8 as u16;
        let new_state = original_state & 0xF1FF | new_value << 9;
        self.write_u16(0x00, new_state)
    }

    pub fn enable_all_channels(&mut self) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_value: u16 = 0b111;
        let new_state = original_state & 0x8fff | new_value << 12;
        self.write_u16(0x00, new_state)
    }

    pub fn disable_all_channels(&mut self) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_state = original_state & 0x8fff;
        self.write_u16(0x00, new_state)
    }

    pub fn enable_channel1(&mut self) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_state = original_state & 0xbfff | 1 << 14;
        self.write_u16(0x00, new_state)
    }

    pub fn disable_channel1(&mut self) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_state = original_state & 0xbfff;
        self.write_u16(0x00, new_state)
    }

    pub fn enable_channel2(&mut self) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_state = original_state & 0xdfff | 1 << 13;
        self.write_u16(0x00, new_state)
    }

    pub fn disable_channel2(&mut self) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_state = original_state & 0xdfff;
        self.write_u16(0x00, new_state)
    }

    pub fn enable_channel3(&mut self) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_state = original_state & 0xefff | 1 << 12;
        self.write_u16(0x00, new_state)
    }

    pub fn disable_channel3(&mut self) -> Result<(), Error> {
        let original_state = self.read_u16(0x00)?;
        let new_state = original_state & 0xefff;
        self.write_u16(0x00, new_state)
    }

    #[inline]
    fn read_shunt_volt(&mut self, reg: u8) -> Result<i32, Error> {
        let raw_value = self.read_u16(reg)?;
        let signed_actual = (raw_value as i16) >> 3;
        Ok(signed_actual as i32 * 40)
    }

    #[inline]
    fn read_bus_volt(&mut self, reg: u8) -> Result<i32, Error> {
        let raw_value = self.read_u16(reg)?;
        let signed_actual = (raw_value as i16) >> 3;
        Ok(signed_actual as i32 * 8)
    }

    /// Shunt voltage channel 1, in microvolt(uV).
    pub fn shunt_channel1(&mut self) -> Result<i32, Error> {
        self.read_shunt_volt(0x01)
    }

    /// Shunt voltage channel 1, in milivolt(mV).
    pub fn bus_channel1(&mut self) -> Result<i32, Error> {
        self.read_bus_volt(0x02)
    }

    /// Shunt voltage channel 2, in microvolt(uV).
    pub fn shunt_channel2(&mut self) -> Result<i32, Error> {
        self.read_shunt_volt(0x03)
    }

    /// Shunt voltage channel 2, in milivolt(mV).
    pub fn bus_channel2(&mut self) -> Result<i32, Error> {
        self.read_bus_volt(0x04)
    }
    /// Shunt voltage channel 3, in microvolt(uV).
    pub fn shunt_channel3(&mut self) -> Result<i32, Error> {
        self.read_shunt_volt(0x05)
    }

    /// Shunt voltage channel 3, in milivolt(mV).
    pub fn bus_channel3(&mut self) -> Result<i32, Error> {
        self.read_bus_volt(0x06)
    }

    /// Calculates current at channel 1 based on the resistor value provided.
    /// 
    /// in milli-Amp
    pub fn current_channel1(&mut self) -> Result<i32, Error> {
        let voltage = self.shunt_channel1()?;
        let resistor = self.shunt_r1 as i32;
        Ok(voltage / resistor)
    }

    /// Calculates current at channel 2 based on the resistor value provided.
    /// 
    /// in milli-Amp
    pub fn current_channel2(&mut self) -> Result<i32, Error> {
        let voltage = self.shunt_channel2()?;
        let resistor = self.shunt_r2 as i32;
        Ok(voltage / resistor)
    }

    /// Calculates current at channel 3 based on the resistor value provided.
    /// 
    /// in milli-Amp
    pub fn current_channel3(&mut self) -> Result<i32, Error> {
        let voltage = self.shunt_channel3()?;
        let resistor = self.shunt_r3 as i32;
        Ok(voltage / resistor)
    }
}
