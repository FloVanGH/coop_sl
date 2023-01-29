// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use std::rc::Rc;

use pico_engine_rs::DrawContext;

use slint::*;

#[allow(clippy::all)]
pub mod ui {
    slint::include_modules!();
}

pub fn main() {
    let game = ui::Game::new();
    let drawing_context = Rc::new(DrawContext::new(160, 144, game.get_palette()));

    drawing::setup(&game, &drawing_context);

    let timer = Timer::default();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(33), {
        let weak = game.as_weak();
        move || {
            weak.unwrap().invoke_update();
            weak.unwrap().invoke_draw();
            weak.unwrap().set_frame_buffer(drawing_context.image());
        }
    });

    game.run()
}

mod drawing {
    use std::rc::Rc;

    use slint::*;

    use pico_engine_rs::DrawContext;

    use crate::ui;

    pub fn setup(game: &ui::Game, drawing_context: &Rc<DrawContext>) {
        game.global::<ui::DrawContext>().on_clear({
            let drawing_context = drawing_context.clone();

            move |color| {
                drawing_context.clear(color);
            }
        });
        game.global::<ui::DrawContext>().on_rect({
            let drawing_context = drawing_context.clone();

            move |x0, y0, x1, y1, color| {
                drawing_context.rect(x0, y0, x1, y1, color);
            }
        });

        game.global::<ui::DrawContext>().on_fill_rect({
            let drawing_context = drawing_context.clone();

            move |x0, y0, x1, y1, color| {
                drawing_context.fill_rect(x0, y0, x1, y1, color);
            }
        });

        game.global::<ui::DrawContext>().on_print({
            let drawing_context = drawing_context.clone();

            move |text, x, y, color| drawing_context.print(text, x, y, color)
        });
    }
}
