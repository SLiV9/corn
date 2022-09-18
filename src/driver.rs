//
// Part of corn
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//
// Based on beep2src
// Copyright (c) 2021 MrRafael-dev
// License: MIT
//

use crate::wasm4::*;

/// Musical note frequencies used by tracks.
const TRACK_NOTES: [u16; 37] = [
	130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 250, 260, 280, 290,
	310, 330, 350, 370, 390, 410, 440, 460, 490, 520, 550, 600, 620, 660, 700,
	750, 780, 840, 880, 940, 980, 1000,
];

/// Instruments available for use.
const TRACK_INSTRUMENTS: [u32; 5] = [
	TONE_TRIANGLE | TONE_MODE1, // Triangle
	TONE_PULSE2 | TONE_MODE3,   // Square
	TONE_PULSE2 | TONE_MODE4,   // Pulse wide
	TONE_PULSE2 | TONE_MODE2,   // Pulse narrow
	TONE_PULSE2 | TONE_MODE1,   // Sawtooth
];

const MAX_NUM_DATA_BYTES_PER_MELODY: usize = 1024;

const MAX_NUM_BARS: usize = 16;
const MAX_NUM_PATTERNS: usize = 8;
const MAX_NUM_TRACKS: usize = 3;

pub struct Melody
{
	pub volume: u8,
	pub data: MelodyData,
}

pub struct MelodyData
{
	bar_data: [Bar; MAX_NUM_BARS],
	pattern_data: [[Pattern; MAX_NUM_PATTERNS]; MAX_NUM_TRACKS],
	note_data: [DataByte; MAX_NUM_DATA_BYTES_PER_MELODY],
}

struct Bar(u16);

impl Bar
{
	pub fn features_track(&self, track_offset: usize) -> bool
	{
		((self.0 >> (4 * track_offset)) & 0b1000) > 0
	}

	pub fn pattern_offset_of_track(&self, track_offset: usize) -> usize
	{
		((self.0 >> (4 * track_offset)) & 0b0111) as usize
	}

	pub fn is_end(&self) -> bool
	{
		self.0 == BAR_OPCODE_END
	}
}

const BAR_OPCODE_END: u16 = 0xEEEE;

struct Pattern
{
	start_offset: u16,
	end_offset: u16,
}

struct DataByte(u8);

impl DataByte
{
	fn decode(&self) -> Decoded
	{
		// Given that note offsets 0-36 are all smaller than 48.
		if (self.0 & 0b00110000) > 0
		{
			Decoded::Note {
				offset: self.0 & 0b00111111,
				wait: (self.0 & 0b11000000) >> 6,
			}
		}
		else
		{
			let upper = (self.0 & 0b11000000) >> 6;
			let lower = self.0 & 0b00001111;
			let wait = (upper << 4) | lower;
			Decoded::Wait { wait }
		}
	}
}

enum Decoded
{
	Note
	{
		offset: u8, wait: u8
	},
	Wait
	{
		wait: u8
	},
}

/// Reserved for empty notes.
const TRACK_OPCODE_EMPTY: u8 = 0xFF;
/// This will cut-off the track, reverting it to the beginning.
const TRACK_OPCODE_END: u8 = 0xFE;

/// Number of tones in one track.
const NUM_TONES: usize = 320;

/// # Track
///
/// A *sound track* is basically one fragment of a music.
pub struct Track
{
	/// Next tone index.
	pub next: usize,
	/// Wait time until the next tone.
	pub wait: u16,
	/// Ticks per beat.
	pub ticks: u8,
	/// Instrument used by this track (see `TRACK_INSTRUMENTS`).
	pub instrument: u8,
	/// Variable Flags reserved for opcodes.
	pub flags: (u8, u8),
	/// Soundtrack tones.
	pub tones: [(u8, u8, u8); NUM_TONES],
}

impl Default for Track
{
	fn default() -> Self
	{
		return Track {
			next: 0,
			wait: 0,
			ticks: 1,
			instrument: 0,
			flags: (0, 0),
			tones: [(0, 255, 0); NUM_TONES],
		};
	}
}

impl Track
{
	/// Resets the track back to beginning.
	pub fn reset(&mut self)
	{
		self.next = 0;
		self.wait = 0;
	}

	/// Event responsible for controlling music execution.
	pub fn step(&mut self)
	{
		// When empty, proceed to next tone index...
		if self.wait == 0
		{
			// Musical tone to be played.
			let current_tone: (u8, u8, u8) = self.tones[self.next];
			let note: u8 = current_tone.0;
			let wait: u8 = current_tone.1;

			// Instrument in use.
			let instrument: usize =
				(self.instrument as usize) % TRACK_INSTRUMENTS.len();

			// Play tone...
			if note < TRACK_NOTES.len() as u8
			{
				let freq = TRACK_NOTES[note as usize] as u32;
				let release = (wait * self.ticks) as u32;
				let volume = 50;
				let flags = TRACK_INSTRUMENTS[instrument];
				tone(freq, release, volume, flags);
			}

			// Proceed to next note...
			self.next = (self.next + 1) % self.tones.len();
			self.wait = wait as u16;

			// Redirection...
			if note == TRACK_OPCODE_END
			{
				self.next = 0;
			}
		}
		// Countdown wait time...
		else
		{
			self.wait -= 1;
		}
	}
}
