//
// Part of corn
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

mod corn_title_theme;

use corn_title_theme::*;

pub fn play_title_theme(t: usize, volume: u32)
{
	play_corn_title_theme_track_1(t, volume);
	play_corn_title_theme_track_2(t, volume);
}
