//! SH1107 I2C Interface

use hal;

use super::DisplayInterface;
use crate::{command::Page, Error};

/// SH1107 I2C communication interface
pub struct I2cInterface<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C> I2cInterface<I2C>
where
    I2C: hal::i2c::I2c,
{
    /// Create new sh1107 I2C interface
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self { i2c, addr }
    }
}

impl<I2C> DisplayInterface for I2cInterface<I2C>
where
    I2C: hal::i2c::I2c,
{
    type Error = Error<<I2C as hal::i2c::ErrorType>::Error, ()>;

    fn init(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        // Copy over given commands to new aray to prefix with command identifier
        let mut writebuf: [u8; 8] = [0; 8];
        writebuf[1..=cmds.len()].copy_from_slice(&cmds);

        self.i2c
            .write(self.addr, &writebuf[..=cmds.len()])
            .map_err(Error::Comm)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        // TODO: figure out a way to pass chunklen in, should likely always be 64 for sh1107, but the sh1106 was doing 128
        const CHUNKLEN: usize = 64;

        const BUFLEN: usize = CHUNKLEN + 1;

        // Noop if the data buffer is empty
        if buf.is_empty() {
            return Ok(());
        }

        let mut page = Page::Page0 as u8;

        // Display width plus 4 start bytes
        let mut writebuf: [u8; BUFLEN] = [0; BUFLEN];

        writebuf[0] = 0x40; // Following bytes are data bytes

        for chunk in buf.chunks(CHUNKLEN) {
            // Copy over all data from buffer, leaving the data command byte intact
            writebuf[1..BUFLEN].copy_from_slice(&chunk);

            self.i2c
                .write(
                    self.addr,
                    &[
                        0x00, // Command
                        page, // Page address
                        // Shulltronics mod: change from 2 to 0 (why was is 2?)
                        0x00, // Lower column address
                        0x10, // Upper column address (always zero, base is 10h)
                    ],
                )
                .map_err(Error::Comm)?;

            self.i2c.write(self.addr, &writebuf).map_err(Error::Comm)?;

            page += 1;
        }

        Ok(())
    }
}
