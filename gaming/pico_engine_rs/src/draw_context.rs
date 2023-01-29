// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::{Baseline, Text},
};

use std::cell::RefCell;

use slint::*;

use crate::Display;

pub struct DrawContext {
    palette: ModelRc<Color>,
    display: RefCell<Display>,
}

impl DrawContext {
    pub fn new(width: u32, height: u32, palette: ModelRc<Color>) -> Self {
        Self {
            palette,
            display: RefCell::new(Display::new(width, height)),
        }
    }

    pub fn clear(&self, _color: i32) {
        println!("clear");
        // if let Some(color) = self.palette.row_data(color as usize) {
        //     self.frame_buffer.make_mut_bytes()
        // }
    }

    pub fn rect(&self, x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
        if let Some(color) = self.palette.row_data(color as usize) {
            Rectangle::new(
                Point::new(x0, y0),
                Size::new((x1 - x0).max(0) as u32, (y1 - y0).max(0) as u32),
            )
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(rgb888_from_color(&color))
                    .stroke_width(1)
                    .build(),
            )
            .draw(&mut *self.display.borrow_mut())
            .unwrap();
        }
    }

    pub fn fill_rect(&self, x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
        if let Some(color) = self.palette.row_data(color as usize) {
            Rectangle::new(
                Point::new(x0, y0),
                Size::new((x1 - x0).max(0) as u32, (y1 - y0).max(0) as u32),
            )
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(rgb888_from_color(&color))
                    .build(),
            )
            .draw(&mut *self.display.borrow_mut())
            .unwrap();
        }
    }

    pub fn print(&self, text: SharedString, x: i32, y: i32, color: i32) -> i32 {
        if let Some(color) = self.palette.row_data(color as usize) {
            Text::with_baseline(
                text.as_str(),
                Point::new(x, y),
                MonoTextStyle::new(&FONT_6X10, rgb888_from_color(&color)),
                Baseline::Top,
            )
            .draw(&mut *self.display.borrow_mut())
            .unwrap();

            // todo return next x
        }

        x
    }

    pub fn image(&self) -> Image {
        self.display.borrow().image()
    }
}

fn rgb888_from_color(color: &Color) -> Rgb888 {
    Rgb888::new(color.red(), color.green(), color.blue())
}
