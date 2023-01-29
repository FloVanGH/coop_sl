// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

pub struct Display {
    frame_buffer: slint::SharedPixelBuffer<slint::Rgb8Pixel>,
}

impl Display {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            frame_buffer: slint::SharedPixelBuffer::new(width, height),
        }
    }

    pub fn image(&self) -> slint::Image {
        slint::Image::from_rgb8(self.frame_buffer.clone())
    }
}

impl DrawTarget for Display {
    type Color = Rgb888;

    type Error = String;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        let size = self.size();
        let data = self.frame_buffer.make_mut_bytes();

        for Pixel(p, color) in pixels.into_iter() {
            if p.x >= 0 && p.y >= 0 && p.x < size.width as i32 && p.y < size.height as i32 {
                let index = (p.y as usize * size.width as usize + p.x as usize) * 3;
                data[index] = color.r();
                data[index + 1] = color.g();
                data[index + 2] = color.b();
            }
        }

        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(self.frame_buffer.width(), self.frame_buffer.height())
    }
}
