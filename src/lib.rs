//
// Part of corn
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

mod wasm4;

#[cfg(feature = "buddy-alloc")]
mod alloc;

mod driver;
mod music;

use wasm4::*;

#[no_mangle]
fn update()
{
    unsafe { *DRAW_COLORS = 2 }
    text("Hello from Rust!", 10, 10);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
    }

    unsafe { music::ITS_CORN.step() };

    text("Press X to blink", 16, 90);
}
