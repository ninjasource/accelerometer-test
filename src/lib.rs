#![no_std]
#![allow(dead_code)]

use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use reg::Register;
mod reg;

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
}

impl<CS, PinError> LIS2DW12<CS>
where
    CS: OutputPin<Error = PinError>,
{
    pub fn new(cs: CS) -> Self {
        Self { cs }
    }

    pub fn get_device_id<E>(
        &mut self,
        spi: &mut impl Transfer<u8, Error = E>,
    ) -> Result<u8, Error<E, PinError>> {
        self.read_from(spi, Register::WHO_AM_I)
    }

    fn write_to<E>(
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

    fn read_from<E>(
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
