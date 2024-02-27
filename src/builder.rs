//! Interface factory
//!
//! This is the easiest way to create a driver instance. You can set various parameters of the
//! driver and give it an interface to use. The builder will return a
//! [`mode::RawMode`](../mode/raw/struct.RawMode.html) object which you should coerce to a richer
//! display mode, like [mode::Graphics](../mode/graphics/struct.GraphicsMode.html) for drawing
//! primitives and text.
//!
//! # Examples
//!
//! Connect over SPI with default rotation (0 deg) and size (128x64):
//!
//! ```rust,ignore
//! let spi = /* SPI interface from your HAL of choice */;
//! let dc = /* GPIO data/command select pin */;
//!
//! Builder::new().connect_spi(spi, dc);
//! ```
//!
//! Connect over I2C, changing lots of options
//!
//! ```rust,ignore
//! let i2c = /* I2C interface from your HAL of choice */;
//!
//! Builder::new()
//!     .with_rotation(DisplayRotation::Rotate180)
//!     .with_i2c_addr(0x3D)
//!     .with_size(DisplaySize::Display128x32)
//!     .connect_i2c(i2c);
//! ```
//!
//! The above examples will produce a [RawMode](../mode/raw/struct.RawMode.html) instance
//! by default. You need to coerce them into a mode by specifying a type on assignment. For
//! example, to use [`GraphicsMode` mode](../mode/graphics/struct.GraphicsMode.html):
//!
//! ```rust,ignore
//! let spi = /* SPI interface from your HAL of choice */;
//! let dc = /* GPIO data/command select pin */;
//!
//! let display: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();
//! ```

use core::marker::PhantomData;
use core::convert::Infallible;
use hal::{
    self,
    digital::ErrorType,
    digital::OutputPin,
};

use crate::{
    displayrotation::DisplayRotation,
    displaysize::DisplaySize,
    interface::{I2cInterface, SpiInterface},
    mode::{displaymode::DisplayMode, raw::RawMode},
    properties::DisplayProperties,
};

/// Builder struct. Driver options and interface are set using its methods.
#[derive(Clone, Copy)]
pub struct Builder {
    display_size: DisplaySize,
    rotation: DisplayRotation,
    i2c_addr: u8,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Create new builder with a default size of 128 x 64 pixels and no rotation.
    pub fn new() -> Builder {
        Builder {
            display_size: DisplaySize::Display128x64,
            rotation: DisplayRotation::Rotate0,
            i2c_addr: 0x3c,
        }
    }
}

impl Builder {
    /// Set the size of the display. Supported sizes are defined by [DisplaySize].
    pub fn with_size(self, display_size: DisplaySize) -> Self {
        Self {
            display_size,
            ..self
        }
    }

    /// Set the I2C address to use. Defaults to 0x3C which is the most common address.
    /// The other address specified in the datasheet is 0x3D. Ignored when using SPI interface.
    pub fn with_i2c_addr(self, i2c_addr: u8) -> Self {
        Self { i2c_addr, ..self }
    }

    /// Set the rotation of the display to one of four values. Defaults to no rotation.
    pub fn with_rotation(self, rotation: DisplayRotation) -> Self {
        Self { rotation, ..self }
    }

    /// Finish the builder and use I2C to communicate with the display
    pub fn connect_i2c<I2C>(self, i2c: I2C) -> DisplayMode<RawMode<I2cInterface<I2C>>>
    where
        I2C: hal::i2c::I2c,
    {
        let properties = DisplayProperties::new(
            I2cInterface::new(i2c, self.i2c_addr),
            self.display_size,
            self.rotation,
        );
        DisplayMode::<RawMode<I2cInterface<I2C>>>::new(properties)
    }

    /// Finish the builder and use SPI to communicate with the display
    ///
    /// If the Chip Select (CS) pin is not required, [`NoOutputPin`] can be used as a dummy argument
    ///
    /// [`NoOutputPin`]: ./struct.NoOutputPin.html
    pub fn connect_spi<SPI, DC, CS>(
        self,
        spi: SPI,
        dc: DC,
        cs: CS,
    ) -> DisplayMode<RawMode<SpiInterface<SPI, DC, CS>>>
    where
        SPI: hal::spi::SpiDevice,
        DC: OutputPin,
        CS: OutputPin,
    {
        let properties = DisplayProperties::new(
            SpiInterface::new(spi, dc, cs),
            self.display_size,
            self.rotation,
        );
        DisplayMode::<RawMode<SpiInterface<SPI, DC, CS>>>::new(properties)
    }
}

/// Represents an unused output pin.
#[derive(Clone, Copy)]
pub struct NoOutputPin {
    _m: PhantomData<()>,
}

impl NoOutputPin {
    /// Create a new instance of `NoOutputPin`
    pub fn new() -> Self {
        Self { _m: PhantomData }
    }
}

impl ErrorType for NoOutputPin {
    type Error = Infallible;
}

impl OutputPin for NoOutputPin  {
    // type Error = PinE;
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::NoOutputPin;
    use embedded_hal::digital::OutputPin;

    // enum SomeError {}

    struct SomeDriver<P: OutputPin> {
        #[allow(dead_code)]
        p: P,
    }

    #[test]
    fn test_output_pin() {
        let p = NoOutputPin::new();
        let _d = SomeDriver { p };

        assert!(true);
    }
}
