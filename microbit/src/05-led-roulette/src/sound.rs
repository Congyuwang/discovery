use microbit::{
    hal::{
        gpio::{Disconnected, Level, Output, Pin, PushPull},
        pwm,
    },
    pac::PWM0,
};

const MAX_DUTY: u16 = 256;
const SAMPLE_FREQ: u16 = 62500;
const CHANNEL: microbit::hal::pwm::Channel = microbit::hal::pwm::Channel::C0;

// notes
#[link_section = ".notes"]
static B1: [u16; 127] = [
    0x40, 0x43, 0x46, 0x49, 0x4c, 0x4f, 0x52, 0x55, 0x58, 0x5b, 0x5e, 0x61, 0x63, 0x66, 0x68, 0x6b,
    0x6d, 0x6f, 0x71, 0x73, 0x75, 0x77, 0x78, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7e, 0x7f, 0x7f, 0x7f,
    0x7f, 0x7f, 0x7f, 0x7f, 0x7e, 0x7d, 0x7c, 0x7b, 0x7a, 0x79, 0x77, 0x76, 0x74, 0x72, 0x70, 0x6e,
    0x6c, 0x6a, 0x67, 0x65, 0x62, 0x5f, 0x5c, 0x5a, 0x57, 0x54, 0x51, 0x4e, 0x4b, 0x47, 0x44, 0x41,
    0x3e, 0x3b, 0x38, 0x34, 0x31, 0x2e, 0x2b, 0x28, 0x25, 0x23, 0x20, 0x1d, 0x1a, 0x18, 0x15, 0x13,
    0x11, 0xf, 0xd, 0xb, 0x9, 0x8, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x1, 0x1, 0x2, 0x3, 0x4, 0x5, 0x7, 0x8, 0xa, 0xc, 0xe, 0x10, 0x12, 0x14, 0x17, 0x19, 0x1c,
    0x1e, 0x21, 0x24, 0x27, 0x2a, 0x2d, 0x30, 0x33, 0x36, 0x39, 0x3c,
];
#[link_section = ".notes"]
static C1: [u16; 119] = [
    0x40, 0x43, 0x46, 0x4a, 0x4d, 0x50, 0x53, 0x57, 0x5a, 0x5d, 0x60, 0x63, 0x65, 0x68, 0x6b, 0x6d,
    0x6f, 0x72, 0x74, 0x75, 0x77, 0x79, 0x7a, 0x7b, 0x7d, 0x7d, 0x7e, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f,
    0x7f, 0x7f, 0x7e, 0x7d, 0x7c, 0x7b, 0x7a, 0x78, 0x76, 0x75, 0x73, 0x70, 0x6e, 0x6c, 0x69, 0x67,
    0x64, 0x61, 0x5e, 0x5b, 0x58, 0x55, 0x52, 0x4f, 0x4b, 0x48, 0x45, 0x41, 0x3e, 0x3a, 0x37, 0x34,
    0x30, 0x2d, 0x2a, 0x27, 0x24, 0x21, 0x1e, 0x1b, 0x18, 0x16, 0x13, 0x11, 0xf, 0xc, 0xa, 0x9,
    0x7, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x2, 0x2, 0x4, 0x5, 0x6,
    0x8, 0xa, 0xb, 0xd, 0x10, 0x12, 0x14, 0x17, 0x1a, 0x1c, 0x1f, 0x22, 0x25, 0x28, 0x2c, 0x2f,
    0x32, 0x35, 0x39, 0x3c,
];
#[link_section = ".notes"]
static C2: [u16; 60] = [
    0x40, 0x46, 0x4d, 0x53, 0x5a, 0x60, 0x65, 0x6a, 0x6f, 0x73, 0x77, 0x7a, 0x7c, 0x7e, 0x7f, 0x80,
    0x7f, 0x7e, 0x7c, 0x7a, 0x77, 0x73, 0x6f, 0x6a, 0x65, 0x60, 0x5a, 0x53, 0x4d, 0x46, 0x40, 0x39,
    0x32, 0x2c, 0x25, 0x20, 0x1a, 0x15, 0x10, 0xc, 0x8, 0x5, 0x3, 0x1, 0x0, 0x0, 0x0, 0x1, 0x3,
    0x5, 0x8, 0xc, 0x10, 0x15, 0x1a, 0x20, 0x25, 0x2c, 0x32, 0x39,
];
#[link_section = ".notes"]
static D1: [u16; 106] = [
    0x40, 0x43, 0x47, 0x4b, 0x4f, 0x52, 0x56, 0x59, 0x5d, 0x60, 0x63, 0x66, 0x69, 0x6c, 0x6f, 0x71,
    0x73, 0x76, 0x78, 0x79, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7e, 0x7d,
    0x7c, 0x7b, 0x79, 0x78, 0x76, 0x73, 0x71, 0x6f, 0x6c, 0x69, 0x66, 0x63, 0x60, 0x5d, 0x59, 0x56,
    0x52, 0x4f, 0x4b, 0x47, 0x43, 0x40, 0x3c, 0x38, 0x34, 0x30, 0x2d, 0x29, 0x26, 0x22, 0x1f, 0x1c,
    0x19, 0x16, 0x13, 0x10, 0xe, 0xc, 0x9, 0x7, 0x6, 0x4, 0x3, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x1, 0x2, 0x3, 0x4, 0x6, 0x7, 0x9, 0xc, 0xe, 0x10, 0x13, 0x16, 0x19, 0x1c, 0x1f, 0x22,
    0x26, 0x29, 0x2d, 0x30, 0x34, 0x38, 0x3c,
];
#[link_section = ".notes"]
static E1: [u16; 95] = [
    0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x63, 0x67, 0x6a, 0x6d, 0x70, 0x73, 0x75,
    0x77, 0x79, 0x7b, 0x7c, 0x7e, 0x7e, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7e, 0x7d, 0x7c, 0x7a, 0x78,
    0x76, 0x74, 0x71, 0x6f, 0x6c, 0x68, 0x65, 0x62, 0x5e, 0x5a, 0x56, 0x52, 0x4e, 0x4a, 0x46, 0x42,
    0x3d, 0x39, 0x35, 0x31, 0x2d, 0x29, 0x25, 0x21, 0x1d, 0x1a, 0x17, 0x13, 0x10, 0xe, 0xb, 0x9,
    0x7, 0x5, 0x3, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x1, 0x3, 0x4, 0x6, 0x8, 0xa, 0xc, 0xf,
    0x12, 0x15, 0x18, 0x1c, 0x1f, 0x23, 0x27, 0x2b, 0x2f, 0x33, 0x37, 0x3b,
];
#[link_section = ".notes"]
static F1: [u16; 89] = [
    0x40, 0x44, 0x49, 0x4d, 0x51, 0x56, 0x5a, 0x5e, 0x62, 0x65, 0x69, 0x6c, 0x6f, 0x72, 0x75, 0x77,
    0x79, 0x7b, 0x7d, 0x7e, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7e, 0x7d, 0x7c, 0x7a, 0x78, 0x76, 0x74,
    0x71, 0x6e, 0x6b, 0x67, 0x64, 0x60, 0x5c, 0x58, 0x53, 0x4f, 0x4b, 0x46, 0x42, 0x3d, 0x39, 0x34,
    0x30, 0x2c, 0x27, 0x23, 0x1f, 0x1b, 0x18, 0x14, 0x11, 0xe, 0xb, 0x9, 0x7, 0x5, 0x3, 0x2, 0x1,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x2, 0x4, 0x6, 0x8, 0xa, 0xd, 0x10, 0x13, 0x16, 0x1a, 0x1d, 0x21,
    0x25, 0x29, 0x2e, 0x32, 0x36, 0x3b,
];
#[link_section = ".notes"]
static G0: [u16; 159] = [
    0x40, 0x42, 0x45, 0x47, 0x4a, 0x4c, 0x4f, 0x51, 0x53, 0x56, 0x58, 0x5a, 0x5d, 0x5f, 0x61, 0x63,
    0x65, 0x67, 0x69, 0x6b, 0x6d, 0x6f, 0x70, 0x72, 0x73, 0x75, 0x76, 0x78, 0x79, 0x7a, 0x7b, 0x7c,
    0x7d, 0x7d, 0x7e, 0x7e, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7e, 0x7e, 0x7d,
    0x7c, 0x7b, 0x7a, 0x79, 0x78, 0x77, 0x76, 0x74, 0x73, 0x71, 0x70, 0x6e, 0x6c, 0x6a, 0x68, 0x66,
    0x64, 0x62, 0x60, 0x5e, 0x5c, 0x59, 0x57, 0x55, 0x52, 0x50, 0x4d, 0x4b, 0x48, 0x46, 0x43, 0x41,
    0x3e, 0x3c, 0x39, 0x37, 0x34, 0x32, 0x2f, 0x2d, 0x2a, 0x28, 0x26, 0x23, 0x21, 0x1f, 0x1d, 0x1b,
    0x19, 0x17, 0x15, 0x13, 0x11, 0xf, 0xe, 0xc, 0xb, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1,
    0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x1, 0x2, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7,
    0x9, 0xa, 0xc, 0xd, 0xf, 0x10, 0x12, 0x14, 0x16, 0x18, 0x1a, 0x1c, 0x1e, 0x20, 0x22, 0x25,
    0x27, 0x29, 0x2c, 0x2e, 0x30, 0x33, 0x35, 0x38, 0x3a, 0x3d,
];
#[link_section = ".notes"]
static G1: [u16; 80] = [
    0x40, 0x45, 0x4a, 0x4e, 0x53, 0x58, 0x5d, 0x61, 0x65, 0x69, 0x6d, 0x70, 0x73, 0x76, 0x79, 0x7b,
    0x7c, 0x7e, 0x7f, 0x7f, 0x80, 0x7f, 0x7f, 0x7e, 0x7c, 0x7b, 0x79, 0x76, 0x73, 0x70, 0x6d, 0x69,
    0x65, 0x61, 0x5d, 0x58, 0x53, 0x4e, 0x4a, 0x45, 0x40, 0x3a, 0x35, 0x31, 0x2c, 0x27, 0x22, 0x1e,
    0x1a, 0x16, 0x12, 0xf, 0xc, 0x9, 0x6, 0x4, 0x3, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x4,
    0x6, 0x9, 0xc, 0xf, 0x12, 0x16, 0x1a, 0x1e, 0x22, 0x27, 0x2c, 0x31, 0x35, 0x3a,
];
#[link_section = ".notes"]
static SI: [u16; 2] = [0x0, 0x0];

pub type Note = (&'static [u16], u16);
pub type Notes = &'static [Note];

// tracks
pub static DI_LO: [Note; 1] = [(&C1, 500)];
pub static DI_HI: [Note; 1] = [(&C2, 800)];
pub static PEPPA: [Note; 12] = [
    (&G1, 500),
    (&E1, 250),
    (&C1, 250),
    (&D1, 500),
    (&G0, 500),
    (&SI, 500),
    (&G0, 250),
    (&B1, 250),
    (&D1, 250),
    (&F1, 250),
    (&E1, 500),
    (&C1, 500),
];

struct Track {
    notes: Notes,
    position: usize,
}

impl Track {
    fn new(notes: Notes) -> Self {
        Self { notes, position: 0 }
    }

    fn next_note(&mut self) -> Option<Note> {
        match self.notes.get(self.position) {
            Some(note) => {
                self.position += 1;
                Some(*note)
            }
            None => None,
        }
    }
}

enum AudioState {
    Disconnected {
        pwm: pwm::Pwm<PWM0>,
        speaker: Pin<Disconnected>,
    },
    Idle {
        pwm: pwm::Pwm<PWM0>,
    },
    Playing {
        pwm: pwm::Pwm<PWM0>,
        track: Track,
    },
}

impl AudioState {
    pub fn new(pwm: PWM0, speaker: Pin<Disconnected>) -> Self {
        Self::Disconnected {
            pwm: pwm::Pwm::new(pwm),
            speaker,
        }
    }

    /// Set track and swtich to playing.
    pub fn set_track(self, notes: Notes) -> Self {
        if let Self::Idle { pwm } = self {
            Self::Playing {
                pwm,
                track: Track::new(notes),
            }
        } else {
            self
        }
    }

    /// Stop pwm, unset track and switch to idle.
    pub fn stop(self) -> Self {
        if let AudioState::Playing { pwm, track: _ } = self {
            pwm.stop();
            AudioState::Idle { pwm }
        } else {
            self
        }
    }

    /// Connect speaker pin to pwm generator.
    ///
    /// Switch from Disconnected to Idle
    pub fn connect(self) -> Self {
        if let Self::Disconnected { pwm, speaker } = self {
            let speaker = speaker.into_push_pull_output(Level::Low);
            Self::enable(&pwm, speaker);
            Self::Idle { pwm }
        } else {
            self
        }
    }

    /// Disconnect speaker pin to save power.
    ///
    /// Switch from Idle to disconnected.
    pub fn disconnect(self) -> Self {
        if let AudioState::Idle { mut pwm } = self {
            pwm.disable();
            let speaker = pwm.clear_output_pin(CHANNEL).unwrap().into_disconnected();
            AudioState::Disconnected { pwm, speaker }
        } else {
            self
        }
    }

    #[inline]
    fn pwm(&self) -> &pwm::Pwm<PWM0> {
        match self {
            AudioState::Disconnected { pwm, speaker: _ } => pwm,
            AudioState::Idle { pwm } => pwm,
            AudioState::Playing { pwm, track: _ } => pwm,
        }
    }

    /// Play next note.
    ///
    /// Won't do anything if currently not 'Playing'.
    ///
    /// return (self, done)
    fn play_next_note(self) -> (Self, bool) {
        if let Self::Playing { pwm, mut track } = self {
            if let Some(note) = track.next_note() {
                let pwm = Self::play_note(pwm, note);
                (Self::Playing { pwm, track }, false)
            } else {
                (Self::Playing { pwm, track }, true)
            }
        } else {
            (self, false)
        }
    }

    /// Silent note is treated differently with refresh instead of loop.
    fn play_note(pwm: pwm::Pwm<PWM0>, (note, t_ms): Note) -> pwm::Pwm<PWM0> {
        let repeat = Self::loops(t_ms, note.len());
        if note == &SI {
            pwm.set_loop(pwm::Loop::Times(1));
            pwm.set_seq_end_delay(pwm::Seq::Seq0, repeat as u32);
        } else {
            pwm.set_loop(pwm::Loop::Times(repeat));
            pwm.set_seq_end_delay(pwm::Seq::Seq0, 0);
        }
        let (s0, s1) = note.split_at(note.len() / 2);
        let (_, _, pwm) = pwm.load(Some(s0), Some(s1), true).unwrap().split();
        pwm
    }

    /// set channel pins and enable pwm.
    fn enable(pwm: &pwm::Pwm<PWM0>, speaker: Pin<Output<PushPull>>) {
        pwm.set_counter_mode(pwm::CounterMode::Up)
            .set_seq_refresh(pwm::Seq::Seq0, 0)
            .set_seq_end_delay(pwm::Seq::Seq0, 0)
            .set_prescaler(pwm::Prescaler::Div1)
            .set_load_mode(pwm::LoadMode::Common)
            .enable_channel(CHANNEL)
            .enable_interrupt(pwm::PwmEvent::LoopsDone)
            .set_output_pin(CHANNEL, speaker)
            .set_max_duty(MAX_DUTY)
            .enable();
    }

    #[inline]
    fn loops(t_ms: u16, sample_len: usize) -> u16 {
        (t_ms as u32 * SAMPLE_FREQ as u32 / 1000 as u32 / sample_len as u32).max(1) as u16
    }
}

pub struct Sound(Option<AudioState>);

impl Sound {
    pub fn new(pwm: PWM0, speaker: Pin<Disconnected>) -> Self {
        Self(Some(AudioState::new(pwm, speaker)))
    }

    /// set track and start playing.
    /// If currently playing, stop this track.
    pub fn play_track(&mut self, track: Notes) {
        // stop the playing track.
        if let Some(AudioState::Playing { .. }) = &self.0 {
            let state = self.0.take().unwrap();
            self.0.replace(state.stop());
        }

        // if disconnected, connect.
        // if idle, set track.
        // if track set, play next note.
        loop {
            let state = self.0.take().unwrap();
            match &state {
                AudioState::Disconnected { .. } => self.0.replace(state.connect()),
                AudioState::Idle { .. } => self.0.replace(state.set_track(track)),
                AudioState::Playing { .. } => {
                    let (state, _) = state.play_next_note();
                    self.0.replace(state);
                    break;
                }
            };
        }
    }

    /// handles LOOPS_DONE event.
    pub fn handle_interrupt(&mut self) {
        // reset event
        self.0
            .as_ref()
            .unwrap()
            .pwm()
            .reset_event(pwm::PwmEvent::LoopsDone);

        // LOOPS_DONE

        // if track unfinished, play next note.
        // if track finished, stop playing.
        // if idle, disconnect.
        // if disconnected, do nothing.
        loop {
            let state = self.0.take().unwrap();
            match &state {
                AudioState::Playing { .. } => {
                    let (state, done) = state.play_next_note();
                    if done {
                        self.0.replace(state.stop());
                        // go on to execute disconnect
                    } else {
                        self.0.replace(state);
                        break;
                    }
                }
                AudioState::Idle { .. } => {
                    self.0.replace(state.disconnect());
                    break;
                }
                AudioState::Disconnected { .. } => {
                    self.0.replace(state);
                    break;
                }
            };
        }
    }
}
