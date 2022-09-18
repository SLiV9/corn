use crate::wasm4::*;

/// Soundtrack: *corn_title_theme*
pub fn play_corn_title_theme_track_1(t: usize, volume: u32) {
	let sequence = [3,2,1,0];
	match sequence[(t / 128) % sequence.len()] {
		1 => play_corn_title_theme_track_1_pattern_1(t, volume),
		2 => play_corn_title_theme_track_1_pattern_2(t, volume),
		3 => play_corn_title_theme_track_1_pattern_3(t, volume),
		_ => (),
	}
}

fn play_corn_title_theme_track_1_pattern_1(t: usize, volume: u32) {
	let tt = t % 128;
	match tt {
		8 => {
			tone(260, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(200, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		24 => {
			tone(260, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(200, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		40 => {
			tone(1000, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(660, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(520, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		48 => {
			tone(660, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(520, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		_ => (),
	}
}

fn play_corn_title_theme_track_1_pattern_2(t: usize, volume: u32) {
	let tt = t % 128;
	match tt {
		24 => {
			tone(490, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		40 => {
			tone(700, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		72 => {
			tone(700, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		88 => {
			tone(660, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		104 => {
			tone(700, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		_ => (),
	}
}

fn play_corn_title_theme_track_1_pattern_3(t: usize, volume: u32) {
	let tt = t % 128;
	match tt {
		24 => {
			tone(260, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(250, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		40 => {
			tone(1000, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(980, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(780, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(700, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		56 => {
			tone(700, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
			tone(780, 8 << 8, volume * 60 / 100, TONE_PULSE2 | TONE_MODE3);
		}
		_ => (),
	}
}

pub fn play_corn_title_theme_track_2(t: usize, volume: u32) {
	let sequence = [1,1,1,1];
	match sequence[(t / 128) % sequence.len()] {
		1 => play_corn_title_theme_track_2_pattern_1(t, volume),
		_ => (),
	}
}

fn play_corn_title_theme_track_2_pattern_1(t: usize, volume: u32) {
	let tt = t % 128;
	match tt {
		8 => {
			tone(130, 8 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		24 => {
			tone(130, 8 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		40 => {
			tone(520, 8 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		48 => {
			tone(390, 8 << 8, volume * 60 / 100, TONE_TRIANGLE | TONE_MODE1);
		}
		_ => (),
	}
}


