use crate::wasm4;
use crate::FRAMES_PER_TICK;
use heapless::Vec;

#[derive(Clone, Copy)]
pub enum Wave {
    Pulse1,
    Pulse2,
    Triangle,
    Noise,
}

impl Wave {
    pub const fn flags(self) -> u32 {
        match self {
            Wave::Pulse1 => wasm4::TONE_PULSE1 | wasm4::TONE_MODE2,
            Wave::Pulse2 => wasm4::TONE_PULSE2 | wasm4::TONE_MODE2,
            Wave::Triangle => wasm4::TONE_TRIANGLE,
            Wave::Noise => wasm4::TONE_NOISE,
        }
    }
}

#[derive(Clone, Copy)]
enum NoteLength {
    Whole,
    Half,
    Quarter,
    QuarterDot,
    Eighth,
}

impl NoteLength {
    pub const fn value(self) -> u32 {
        match self {
            NoteLength::Whole => 8,
            NoteLength::Half => 4,
            NoteLength::QuarterDot => 3,
            NoteLength::Quarter => 2,
            NoteLength::Eighth => 1,
        }
    }
}

#[derive(Clone, Copy)]
struct Frequency(Option<u16>);

#[derive(Clone, Copy)]
struct Note {
    frequency: Frequency,
    length: NoteLength,
}

impl Note {
    pub const fn new(frequency: Frequency, length: NoteLength) -> Self {
        Self { frequency, length }
    }
}

const SEQUENCE_LENGTH: usize = 8;
#[derive(Clone)]
pub struct Sequence {
    notes: Vec<Note, SEQUENCE_LENGTH>,
}

impl Sequence {
    const fn new(notes: Vec<Note, SEQUENCE_LENGTH>) -> Self {
        Self { notes }
    }

    pub fn length(&self) -> u32 {
        self.notes
            .iter()
            .fold(0, |acc, note| acc + note.length.value())
    }

    fn get_note(&self, time: u32) -> Option<Note> {
        if time == 0 {
            return self.notes.get(0).copied();
        }
        let mut counter = 0;
        for note in &self.notes {
            counter += note.length.value();
            if counter >= time {
                return Some(*note);
            }
        }
        None
    }

    pub fn gen_pattern(variant: u8, random_value: f32) -> Self {
        let mut notes: Vec<Note, SEQUENCE_LENGTH> = Vec::new();
        let sanity_check = match variant {
            0 => {
                if random_value < 0.1 {
                    notes.extend_from_slice(&[
                        Note::new(Frequency(Some(117)), NoteLength::Half),
                        Note::new(Frequency(Some(139)), NoteLength::Quarter),
                        Note::new(Frequency(Some(156)), NoteLength::Half),
                    ])
                } else if random_value < 0.2 {
                    notes.extend_from_slice(&[
                        Note::new(Frequency(Some(139)), NoteLength::Quarter),
                        Note::new(Frequency(Some(185)), NoteLength::Half),
                    ])
                } else if random_value < 0.3 {
                    notes.extend_from_slice(&[
                        Note::new(Frequency(Some(185)), NoteLength::Quarter),
                        Note::new(Frequency(Some(208)), NoteLength::Quarter),
                        Note::new(Frequency(Some(156)), NoteLength::Half),
                    ])
                } else if random_value < 0.4 {
                    notes.extend_from_slice(&[
                        Note::new(Frequency(Some(139)), NoteLength::Quarter),
                        Note::new(Frequency(Some(117)), NoteLength::Half),
                        Note::new(Frequency(Some(117)), NoteLength::Quarter),
                        Note::new(Frequency(None), NoteLength::Quarter),
                        Note::new(Frequency(Some(157)), NoteLength::Quarter),
                    ])
                } else if random_value < 0.5 {
                    notes.extend_from_slice(&[
                        Note::new(Frequency(None), NoteLength::Quarter),
                        Note::new(Frequency(Some(156)), NoteLength::Quarter),
                        Note::new(Frequency(Some(139)), NoteLength::Half),
                    ])
                } else if random_value < 0.6 {
                    notes.extend_from_slice(&[
                        Note::new(Frequency(Some(208)), NoteLength::Eighth),
                        Note::new(Frequency(Some(233)), NoteLength::Eighth),
                        Note::new(Frequency(Some(277)), NoteLength::Quarter),
                        Note::new(Frequency(Some(311)), NoteLength::Quarter),
                        Note::new(Frequency(Some(370)), NoteLength::Eighth),
                        Note::new(Frequency(Some(311)), NoteLength::Eighth),
                        Note::new(Frequency(Some(233)), NoteLength::Quarter),
                        Note::new(Frequency(Some(208)), NoteLength::Eighth),
                    ])
                } else if random_value < 0.7 {
                    notes.extend_from_slice(&[
                        Note::new(Frequency(Some(208)), NoteLength::Quarter),
                        Note::new(Frequency(Some(156)), NoteLength::Quarter),
                        Note::new(Frequency(Some(185)), NoteLength::Quarter),
                        Note::new(Frequency(Some(139)), NoteLength::Half),
                    ])
                } else if random_value < 0.8 {
                    notes.extend_from_slice(&[
                        Note::new(Frequency(Some(117)), NoteLength::Quarter),
                        Note::new(Frequency(Some(139)), NoteLength::Half),
                        Note::new(Frequency(Some(156)), NoteLength::Quarter),
                        Note::new(Frequency(Some(185)), NoteLength::Half),
                    ])
                } else {
                    notes.extend_from_slice(&[Note::new(Frequency(None), NoteLength::Quarter)])
                }
            }
            10 => notes.extend_from_slice(&[Note::new(Frequency(Some(1000)), NoteLength::Quarter)]),
            11 => notes.extend_from_slice(&[
                Note::new(Frequency(Some(831)), NoteLength::Eighth),
                Note::new(Frequency(Some(740)), NoteLength::Eighth),
                Note::new(Frequency(Some(622)), NoteLength::Eighth),
                Note::new(Frequency(Some(554)), NoteLength::Eighth),
                Note::new(Frequency(Some(466)), NoteLength::Eighth),
                Note::new(Frequency(Some(415)), NoteLength::Eighth),
                Note::new(Frequency(Some(370)), NoteLength::Eighth),
                Note::new(Frequency(Some(311)), NoteLength::Eighth),
            ]),
            _ => notes.extend_from_slice(&[Note::new(Frequency(None), NoteLength::Half)]),
        };
        sanity_check.expect("Tried to generate a sequence longer than SEQUENCE_LENGTH");
        Self::new(notes)
    }
}
// FREQUENCIES
// 110 |-117-| 123 130 |-139-| 147 |-156-| 165 175 |-185-| 196 |-208-| 220
// 220 |-233-| 247 262 |-277-| 294 |-311-| 330 349 |-370-| 392 |-415-| 440
// 440 |-466-| 494 523 |-554-| 587 |-622-| 659 698 |-740-| 784 |-831-| 880
pub struct Channel {
    time: u32,
    wait_time: u32,
    pattern: Option<Sequence>,
    instrument: Wave,
}

impl Channel {
    pub const fn new(instrument: Wave) -> Self {
        Self {
            time: 0,
            wait_time: 0,
            pattern: None,
            instrument,
        }
    }

    pub fn try_set_pattern(&mut self, pattern: Sequence) {
        if self.pattern.is_none() {
            self.pattern = Some(pattern);
        }
    }

    pub fn update(&mut self, delta_time: u32) {
        if let Some(pattern) = &self.pattern {
            self.time += delta_time;
            if self.wait_time > delta_time {
                self.wait_time -= delta_time;
            } else if self.time >= pattern.length() {
                self.pattern = None;
                self.time = 0;
                self.wait_time = 0;
            } else {
                let note = pattern.get_note(self.time).expect("Note is None");
                self.wait_time = note.length.value();
                self.play_note(note.frequency, note.length);
            }
        }
    }

    fn play_note(&self, frequency: Frequency, length: NoteLength) {
        let len: u8 = unsafe {
            (u32::from(FRAMES_PER_TICK) * length.value())
                .min(255)
                .try_into()
                .unwrap_unchecked()
        };
        if let Some(frequency) = frequency.0 {
            tone(frequency, None, len / 2, Some(len / 2), 25, self.instrument);
        }
    }
}
pub struct Arrangement {
    time: u32,
    triangle: Option<Channel>,
    square1: Option<Channel>,
    square2: Option<Channel>,
    noise: Option<Channel>,
}

impl Arrangement {
    pub const fn new(
        triangle: Option<Channel>,
        square1: Option<Channel>,
        square2: Option<Channel>,
        noise: Option<Channel>,
    ) -> Self {
        Self {
            time: 0,
            triangle,
            square1,
            square2,
            noise,
        }
    }

    pub fn try_add_pattern(&mut self, channel: Wave, pattern: Sequence) {
        match channel {
            Wave::Triangle => {
                if let Some(triangle) = &mut self.triangle {
                    triangle.try_set_pattern(pattern);
                }
            }
            Wave::Pulse1 => {
                if let Some(square1) = &mut self.square1 {
                    square1.try_set_pattern(pattern);
                }
            }
            Wave::Pulse2 => {
                if let Some(square2) = &mut self.square2 {
                    square2.try_set_pattern(pattern);
                }
            }
            Wave::Noise => {
                if let Some(noise) = &mut self.noise {
                    noise.try_set_pattern(pattern);
                }
            }
        }
    }

    pub fn update(&mut self, delta_time: u32) {
        self.time += delta_time;
        if let Some(triangle) = &mut self.triangle {
            triangle.update(delta_time);
        }
        if let Some(square1) = &mut self.square1 {
            square1.update(delta_time);
        }
        if let Some(square2) = &mut self.square2 {
            square2.update(delta_time);
        }
        if let Some(noise) = &mut self.noise {
            noise.update(delta_time);
        }
    }
}

fn tone(
    freq_first: u16,
    freq_last: Option<u16>,
    duration: u8,
    release: Option<u8>,
    volume: u32,
    instrument: Wave,
) {
    let freq_last = freq_last.unwrap_or(0);
    let release = u32::from(release.unwrap_or(0));
    let duration = u32::from(duration) | (release << 8);
    wasm4::tone(
        u32::from(freq_first) | (u32::from(freq_last) << 16),
        duration,
        volume,
        instrument.flags(),
    );
}
