// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use embedded_graphics::{draw_target::*, geometry::Size, pixelcolor::*, prelude::*, Pixel};
use slint::platform::software_renderer::*;

/// Defines a pixel color that is used to draw to the display buffer of the PSP.
///
/// `Color` represents a RGBA888 color.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Color(pub u32);

impl Color {
    /// Gets the read value of the color.
    pub fn r(&self) -> u8 {
        (self.0 & 0x000000FF) as u8
    }

    /// Gets the green value of the color.
    pub fn g(&self) -> u8 {
        ((self.0 & 0x0000FF00) >> 8) as u8
    }

    /// Gets the blue value of the color.
    pub fn b(&self) -> u8 {
        ((self.0 & 0x00FF0000) >> 16) as u8
    }

    /// Gets the alpha channel of the color.
    pub fn a(&self) -> u8 {
        ((self.0 & 0x00FF0000) >> 24) as u8
    }
}

impl TargetPixel for Color {
    fn blend(&mut self, color: PremultipliedRgbaColor) {
        let a = (u8::MAX - color.alpha) as u16;
        *self = Self::from_rgb(
            (self.r() as u16 * a / 255) as u8 + color.red,
            (self.g() as u16 * a / 255) as u8 + color.green,
            (self.b() as u16 * a / 255) as u8 + color.blue,
        );
    }

    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r) as u32) | ((g as u32) << 8) | ((b as u32) << 16))
    }
}

/// Helper frame buffer wrapper to use with embedded-graphics.
pub struct PspFrameBuffer<'a> {
    buffer: &'a mut [Color],
}

impl<'a> PspFrameBuffer<'a> {
    /// Creates a new buffer.
    pub fn new(buffer: &'a mut [Color]) -> Self {
        Self { buffer }
    }

    fn draw_pixel(&mut self, pixel: Pixel<Rgb888>) -> Result<(), core::convert::Infallible> {
        let Pixel(coord, color) = pixel;

        if let Ok((x @ 0..=psp::SCREEN_WIDTH, y @ 0..=psp::SCREEN_HEIGHT)) = coord.try_into() {
            unsafe {
                let ptr = self
                    .buffer
                    .as_mut_ptr()
                    .offset(x as isize)
                    .offset((y * psp::BUF_WIDTH) as isize);

                *ptr = Color(
                    (color.r() as u32) | ((color.g() as u32) << 8) | ((color.b() as u32) << 16),
                );
            }
        }

        Ok(())
    }
}

impl<'a> DrawTarget for PspFrameBuffer<'a> {
    type Color = Rgb888;
    type Error = alloc::string::String;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for p in pixels.into_iter() {
            self.draw_pixel(p)
                .map_err(|_| alloc::string::String::new())?;
        }

        Ok(())
    }
}

impl<'a> OriginDimensions for PspFrameBuffer<'a> {
    fn size(&self) -> Size {
        Size::new(psp::SCREEN_WIDTH, psp::SCREEN_HEIGHT)
    }
}
