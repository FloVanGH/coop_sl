// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use slint::platform::software_renderer::*;

/// Defines a color that maps to the rgb of `orbclient::Color`.
///
/// `Color` represents a RGBA888 color.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Color(pub u32);

impl Color {
    /// Gets the read value of the color.
    pub fn r(&self) -> u8 {
        ((self.0 & 0x00FF0000) >> 16) as u8
    }

    /// Gets the green value of the color.
    pub fn g(&self) -> u8 {
        ((self.0 & 0x0000FF00) >> 8) as u8
    }

    /// Gets the blue value of the color.
    pub fn b(&self) -> u8 {
        (self.0 & 0x000000FF) as u8
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
        Self(0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }
}
