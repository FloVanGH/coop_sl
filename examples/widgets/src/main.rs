#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

#![cfg_attr(not(feature = "std"), feature(default_alloc_error_handler))]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg(feature = "std")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    Widgets::new().run()
}

#[cfg(feature = "rpi")]
#[rp_pico::entry]
fn main() -> ! {
    slint_rpi_pico::init(||  Widgets::new().run());
   

    panic!("Should not quit")
}
