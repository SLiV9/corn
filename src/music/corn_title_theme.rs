use crate::wasm4::*;

/// Soundtrack: *corn_title_theme*
pub fn play_corn_title_theme_track_1(t: usize, volume: u32) {
	let sequence = [3,1,3,1];
	match sequence[(t / 224) % sequence.len()] {
		1 => play_corn_title_theme_track_1_pattern_1(t, volume),
		2 => play_corn_title_theme_track_1_pattern_2(t, volume),
		3 => play_corn_title_theme_track_1_pattern_3(t, volume),
		_ => (),
	}
}

fn play_corn_title_theme_track_1_pattern_1(t: usize, volume: u32) {
	let tt = t % 224;
	match tt {
		7 => {
			tone(260, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		21 => {
			tone(260, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		35 => {
			tone(660, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		49 => {
			tone(520, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		_ => (),
	}
}

fn play_corn_title_theme_track_1_pattern_2(t: usize, volume: u32) {
	let tt = t % 224;
	match tt {
		42 => {
			tone(490, 14 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		70 => {
			tone(700, 14 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		126 => {
			tone(700, 14 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		154 => {
			tone(660, 14 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		182 => {
			tone(700, 14 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		_ => (),
	}
}

fn play_corn_title_theme_track_1_pattern_3(t: usize, volume: u32) {
	let tt = t % 224;
	match tt {
		21 => {
			tone(260, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		35 => {
			tone(1000, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		49 => {
			tone(780, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		133 => {
			tone(490, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		147 => {
			tone(700, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		175 => {
			tone(700, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		189 => {
			tone(660, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		203 => {
			tone(700, 7 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		_ => (),
	}
}

pub fn play_corn_title_theme_track_2(t: usize, volume: u32) {
	let sequence = [1,1,1,1];
	match sequence[(t / 224) % sequence.len()] {
		1 => play_corn_title_theme_track_2_pattern_1(t, volume),
		_ => (),
	}
}

fn play_corn_title_theme_track_2_pattern_1(t: usize, volume: u32) {
	let tt = t % 224;
	match tt {
		7 => {
			tone(130, 7 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		21 => {
			tone(130, 7 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		35 => {
			tone(520, 7 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		49 => {
			tone(390, 7 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		119 => {
			tone(130, 7 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		133 => {
			tone(130, 7 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		147 => {
			tone(520, 7 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		161 => {
			tone(390, 7 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		_ => (),
	}
}


