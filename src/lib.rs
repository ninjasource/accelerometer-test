#![no_std]
#![allow(dead_code)]

use accelerometer::{vector::I16x3, RawAccelerometer};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
mod reg;
use core::fmt::Debug;

pub use crate::reg::*;

#[derive(Debug)]
pub enum Lis2dw12Error<SpiError, PinError> {
    /// SPI communication error
    Spi(SpiError),
    /// CS output pin error
    Pin(PinError),
}

impl<SpiError, PinError> From<SpiError> for Lis2dw12Error<SpiError, PinError> {
    fn from(err: SpiError) -> Self {
        Self::Spi(err)
    }
}

pub struct Lis2dw12<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, SpiError, CS, PinError> Lis2dw12<SPI, CS>
where
    SPI: Transfer<u8, Error = SpiError>,
    CS: OutputPin<Error = PinError>,
{
    pub fn new(spi: SPI, mut cs: CS) -> Result<Self, Lis2dw12Error<SpiError, PinError>> {
        cs.set_high().map_err(Lis2dw12Error::Pin)?;
        Ok(Self { cs, spi })
    }

    pub fn set_low_power_mode(
        &mut self,
        low_power_mode: LowPowerMode,
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        let reset_bits = 0b0000_0011;
        self.reg_reset_bits(Register::CTRL1, reset_bits)?;
        self.reg_set_bits(Register::CTRL1, low_power_mode as u8)?;
        Ok(())
    }

    pub fn set_mode(
        &mut self,
        mode: OperatingMode,
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        let reset_bits = 0b0000_1100;
        let set_bits = (mode as u8) << 2;
        self.reg_reset_bits(Register::CTRL1, reset_bits)?;
        self.reg_set_bits(Register::CTRL1, set_bits)?;
        Ok(())
    }

    pub fn set_low_noise(
        &mut self,
        is_enabled: bool,
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        let bits = 0b0000_0100;
        if is_enabled {
            self.reg_set_bits(Register::CTRL1, bits)?;
        } else {
            self.reg_reset_bits(Register::CTRL1, bits)?;
        }

        Ok(())
    }

    pub fn set_full_scale_selection(
        &mut self,
        full_scale_selection: FullScaleSelection,
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        let reset_bits = 0b0011_0000;
        let set_bits = (full_scale_selection as u8) << 4;
        self.reg_reset_bits(Register::CTRL1, reset_bits)?;
        self.reg_set_bits(Register::CTRL1, set_bits)?;
        Ok(())
    }

    pub fn set_output_data_rate(
        &mut self,
        odr: OutputDataRate,
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        let reset_bits = 0b1111_0000;
        let set_bits = (odr as u8) << 4;
        self.reg_reset_bits(Register::CTRL1, reset_bits)?;
        self.reg_set_bits(Register::CTRL1, set_bits)?;
        Ok(())
    }

    pub fn get_device_id(&mut self) -> Result<u8, Lis2dw12Error<SpiError, PinError>> {
        self.read_reg(Register::WHO_AM_I)
    }

    /*
    pub fn get_sixd<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
    ) -> Result<(), Error<E, PinError>> {
        self.read_reg(spi, Register::SIXD_SRC)
    }*/

    pub fn get_raw(&mut self) -> Result<I16x3, Lis2dw12Error<SpiError, PinError>> {
        let mut buf = [0u8; 6];
        self.read_regs(Register::OUT_X_L, &mut buf)?;

        Ok(I16x3::new(
            ((buf[0] as u16) + ((buf[1] as u16) << 8)) as i16,
            ((buf[2] as u16) + ((buf[3] as u16) << 8)) as i16,
            ((buf[4] as u16) + ((buf[5] as u16) << 8)) as i16,
        ))
    }

    fn read_regs(
        &mut self,
        register: Register,
        buf: &mut [u8],
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        // this flag allows us to call read multiple times and the register will automatically be incremented
        const IF_ADD_INC: u8 = 0b0000_0100;
        self.reg_set_bits(Register::CTRL2, IF_ADD_INC)?;

        self.chip_select().map_err(Lis2dw12Error::Pin)?;
        let request = 0b1000_0000 | register.addr(); // set the read bit

        let result = self.write(request).and_then(|_| {
            for x in buf {
                *x = self.read()?;
            }

            Ok(())
        });

        self.chip_deselect().map_err(Lis2dw12Error::Pin)?;
        self.reg_reset_bits(Register::CTRL2, IF_ADD_INC)?;
        result
    }

    fn reg_set_bits(
        &mut self,
        reg: Register,
        bits: u8,
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        self.modify_reg(reg, |v| v | bits)
    }

    fn reg_reset_bits(
        &mut self,
        reg: Register,
        bits: u8,
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        self.modify_reg(reg, |v| v & !bits)
    }

    fn modify_reg<F>(
        &mut self,
        reg: Register,
        f: F,
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>>
    where
        F: FnOnce(u8) -> u8,
    {
        let r = self.read_reg(reg)?;
        self.write_reg(reg, f(r))?;
        Ok(())
    }

    fn write_reg(
        &mut self,
        register: Register,
        data: u8,
    ) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        self.chip_select().map_err(Lis2dw12Error::Pin)?;
        let result = self.write(register.addr()).and_then(|_| self.write(data));
        self.chip_deselect().map_err(Lis2dw12Error::Pin)?;
        result?;
        Ok(())
    }

    fn read_reg(&mut self, register: Register) -> Result<u8, Lis2dw12Error<SpiError, PinError>> {
        self.chip_select().map_err(Lis2dw12Error::Pin)?;
        let request = 0b1000_0000 | register.addr(); // set the read bit
        let result = self.write(request).and_then(|_| self.read());
        self.chip_deselect().map_err(Lis2dw12Error::Pin)?;
        Ok(result?)
    }

    fn write(&mut self, byte: u8) -> Result<(), Lis2dw12Error<SpiError, PinError>> {
        self.spi.transfer(&mut [byte])?;
        Ok(())
    }

    fn read(&mut self) -> Result<u8, Lis2dw12Error<SpiError, PinError>> {
        let result = self.spi.transfer(&mut [0x00])?[0];
        Ok(result)
    }

    fn chip_select(&mut self) -> Result<(), PinError> {
        self.cs.set_low()
    }

    fn chip_deselect(&mut self) -> Result<(), PinError> {
        self.cs.set_high()
    }
}

impl<SPI, SpiError, CS, PinError> RawAccelerometer<I16x3> for Lis2dw12<SPI, CS>
where
    SPI: Transfer<u8, Error = SpiError>,
    CS: OutputPin<Error = PinError>,
    SpiError: Debug,
    PinError: Debug,
{
    type Error = Lis2dw12Error<SpiError, PinError>;

    /// Get acceleration reading from the accelerometer
    fn accel_raw(&mut self) -> Result<I16x3, accelerometer::Error<Self::Error>> {
        let mut buf = [0u8; 6];
        self.read_regs(Register::OUT_X_L, &mut buf)?;

        Ok(I16x3::new(
            ((buf[0] as u16) + ((buf[1] as u16) << 8)) as i16,
            ((buf[2] as u16) + ((buf[3] as u16) << 8)) as i16,
            ((buf[4] as u16) + ((buf[5] as u16) << 8)) as i16,
        ))
    }
}
