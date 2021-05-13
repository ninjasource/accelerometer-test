#![no_std]
#![allow(dead_code)]

use accelerometer::vector::I16x3;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
mod reg;

pub use crate::reg::*;

#[derive(Debug)]
pub enum Error<SpiError, PinError> {
    /// SPI communication error
    Spi(SpiError),
    /// CS output pin error
    Pin(PinError),
}

impl<SpiError, PinError> From<SpiError> for Error<SpiError, PinError> {
    fn from(err: SpiError) -> Self {
        Self::Spi(err)
    }
}

pub struct LIS2DW12<CS> {
    cs: CS,
    //    x_is_enabled: bool,
    //    x_last_output_data_rate: OutputDataRate,
    //    x_last_operating_mode: OperatingMode,
    //    x_last_low_noise: LowNoise,
}

impl<CS, PinError> LIS2DW12<CS>
where
    CS: OutputPin<Error = PinError>,
{
    pub fn new(mut cs: CS) -> Result<Self, PinError> {
        cs.set_high()?;
        Ok(Self {
            cs,
            //          x_is_enabled: false,
            //          x_last_output_data_rate: OutputDataRate::Off,
            //          x_last_operating_mode: OperatingMode::HighPerformanceMode,
            //          x_last_low_noise: LowNoise::Disabled,
        })
    }

    pub fn set_low_power_mode<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        low_power_mode: LowPowerMode,
    ) -> Result<(), Error<E, PinError>> {
        let reset_bits = 0b0000_0011;
        self.reg_reset_bits(spi, Register::CTRL1, reset_bits)?;
        self.reg_set_bits(spi, Register::CTRL1, low_power_mode as u8)?;
        Ok(())
    }

    pub fn set_mode<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        mode: OperatingMode,
    ) -> Result<(), Error<E, PinError>> {
        let reset_bits = 0b0000_1100;
        let set_bits = (mode as u8) << 2;
        self.reg_reset_bits(spi, Register::CTRL1, reset_bits)?;
        self.reg_set_bits(spi, Register::CTRL1, set_bits)?;
        Ok(())
    }

    pub fn set_low_noise<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        is_enabled: bool,
    ) -> Result<(), Error<E, PinError>> {
        let bits = 0b0000_0100;
        if is_enabled {
            self.reg_set_bits(spi, Register::CTRL1, bits)?;
        } else {
            self.reg_reset_bits(spi, Register::CTRL1, bits)?;
        }

        Ok(())
    }

    pub fn set_full_scale_selection<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        full_scale_selection: FullScaleSelection,
    ) -> Result<(), Error<E, PinError>> {
        let reset_bits = 0b0011_0000;
        let set_bits = (full_scale_selection as u8) << 4;
        self.reg_reset_bits(spi, Register::CTRL1, reset_bits)?;
        self.reg_set_bits(spi, Register::CTRL1, set_bits)?;
        Ok(())
    }

    pub fn set_output_data_rate<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        odr: OutputDataRate,
    ) -> Result<(), Error<E, PinError>> {
        let reset_bits = 0b1111_0000;
        let set_bits = (odr as u8) << 4;
        self.reg_reset_bits(spi, Register::CTRL1, reset_bits)?;
        self.reg_set_bits(spi, Register::CTRL1, set_bits)?;
        Ok(())
    }

    pub fn get_device_id<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
    ) -> Result<u8, Error<E, PinError>> {
        self.read_reg(spi, Register::WHO_AM_I)
    }

    /*
    pub fn get_sixd<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
    ) -> Result<(), Error<E, PinError>> {
        self.read_reg(spi, Register::SIXD_SRC)
    }*/

    pub fn get_raw<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
    ) -> Result<I16x3, Error<E, PinError>> {
        let mut buf = [0u8; 6];
        self.read_regs(spi, Register::OUT_X_L, &mut buf)?;

        Ok(I16x3::new(
            ((buf[0] as u16) + ((buf[1] as u16) << 8)) as i16,
            ((buf[2] as u16) + ((buf[3] as u16) << 8)) as i16,
            ((buf[4] as u16) + ((buf[5] as u16) << 8)) as i16,
        ))
    }

    fn read_regs<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        register: Register,
        buf: &mut [u8],
    ) -> Result<(), Error<E, PinError>> {
        // this flag allows us to call read multiple times and the register will automatically be incremented
        const IF_ADD_INC: u8 = 0b0000_0100;
        self.reg_set_bits(spi, Register::CTRL2, IF_ADD_INC)?;

        self.chip_select().map_err(Error::Pin)?;
        let request = 0b1000_0000 | register.addr(); // set the read bit

        let result = self.write(spi, request).and_then(|_| {
            for x in buf {
                *x = self.read(spi)?;
            }

            Ok(())
        });

        self.chip_deselect().map_err(Error::Pin)?;
        self.reg_reset_bits(spi, Register::CTRL2, IF_ADD_INC)?;
        result
    }

    pub fn get_junk<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
    ) -> Result<u8, Error<E, PinError>> {
        self.read_reg(spi, Register::OUT_X_L)
    }

    fn reg_set_bits<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        reg: Register,
        bits: u8,
    ) -> Result<(), Error<E, PinError>> {
        self.modify_reg(spi, reg, |v| v | bits)
    }

    fn reg_reset_bits<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        reg: Register,
        bits: u8,
    ) -> Result<(), Error<E, PinError>> {
        self.modify_reg(spi, reg, |v| v & !bits)
    }

    fn modify_reg<F, E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        reg: Register,
        f: F,
    ) -> Result<(), Error<E, PinError>>
    where
        F: FnOnce(u8) -> u8,
    {
        let r = self.read_reg(spi, reg)?;
        self.write_reg(spi, reg, f(r))?;
        Ok(())
    }

    fn write_reg<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        register: Register,
        data: u8,
    ) -> Result<(), Error<E, PinError>> {
        self.chip_select().map_err(Error::Pin)?;
        let result = self
            .write(spi, register.addr())
            .and_then(|_| self.write(spi, data));
        self.chip_deselect().map_err(Error::Pin)?;
        result?;
        Ok(())
    }

    fn read_reg<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        register: Register,
    ) -> Result<u8, Error<E, PinError>> {
        self.chip_select().map_err(Error::Pin)?;
        let request = 0b1000_0000 | register.addr(); // set the read bit
        let result = self.write(spi, request).and_then(|_| self.read(spi));
        self.chip_deselect().map_err(Error::Pin)?;
        Ok(result?)
    }

    fn write<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
        byte: u8,
    ) -> Result<(), Error<E, PinError>> {
        spi.transfer(&mut [byte])?;
        Ok(())
    }

    fn read<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
    ) -> Result<u8, Error<E, PinError>> {
        let result = spi.transfer(&mut [0x00])?[0];
        Ok(result)
    }

    fn chip_select(&mut self) -> Result<(), PinError> {
        self.cs.set_low()
    }

    fn chip_deselect(&mut self) -> Result<(), PinError> {
        self.cs.set_high()
    }
}

/*
impl<I2C, E> RawAccelerometer<I16x3> for Lis2dh12<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    E: Debug,
{
    type Error = E;

    /// Get acceleration reading from the accelerometer
    fn accel_raw(&mut self) -> Result<I16x3, Error<E>> {
        let mut buf = [0u8; 6];
        self.read_regs(Register::OUT_X_L, &mut buf)?;

        Ok(I16x3::new(
            (u16(buf[0]) + (u16(buf[1]) << 8)) as i16,
            (u16(buf[2]) + (u16(buf[3]) << 8)) as i16,
            (u16(buf[4]) + (u16(buf[5]) << 8)) as i16,
        ))
    }
}*/
