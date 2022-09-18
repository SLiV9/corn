//
// Part of corn
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

mod wasm4;

#[cfg(feature = "buddy-alloc")]
mod alloc;

mod music;

use wasm4::*;

static mut TICKS: usize = 0;

#[no_mangle]
fn update()
{
	let t = unsafe { TICKS };

	let gamepad = unsafe { *GAMEPAD1 };
	if gamepad & BUTTON_1 != 0
	{
		unsafe { *DRAW_COLORS = 4 }
		music::play_title_theme(t, 100);
	}

	unsafe {
		TICKS += 1;
	}

	text("Press X to blink", 16, 90);
}
