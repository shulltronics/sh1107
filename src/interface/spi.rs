//! sh1107 SPI interface

use hal::{self, digital::OutputPin};

use super::DisplayInterface;
use crate::Error;

/// SPI display interface.
///
/// This combines the SPI peripheral and a data/command pin
pub struct SpiInterface<SPI, DC, CS> {
    spi: SPI,
    dc: DC,
    cs: CS,
}

impl<SPI, DC, CS> SpiInterface<SPI, DC, CS>
where
    SPI: hal::spi::SpiDevice,
    DC: OutputPin,
    CS: OutputPin,
{
    /// Create new SPI interface for communciation with sh1107
    pub fn new(spi: SPI, dc: DC, cs: CS) -> Self {
        Self { spi, dc, cs }
    }
}

impl<SPI, DC, CS> DisplayInterface for SpiInterface<SPI, DC, CS>
where
    SPI: hal::spi::SpiDevice,
    DC: OutputPin,
    CS: OutputPin,
{
    type Error = Error<<SPI as hal::spi::ErrorType>::Error, <CS as hal::digital::ErrorType>::Error>;

    fn init(&mut self) -> Result<(), Self::Error> {
        self.cs.set_high().map_err(Error::Pin)
    }

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        // self.dc.set_low().map_err(Error::Pin)?;

        self.spi.write(&cmds).map_err(Error::Comm)?;

        // self.dc.set_high().map_err(Error::Pin)?;
        self.cs.set_high().map_err(Error::Pin)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;

        // 1 = data, 0 = command
        // self.dc.set_high().map_err(Error::Pin)?;

        self.spi.write(&buf).map_err(Error::Comm)?;

        self.cs.set_high().map_err(Error::Pin)
    }
}
