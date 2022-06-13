#[allow(dead_code)]
pub enum Value {
    Maxima,
    Long,
    DoubleWhole,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
    HundredTwentyEighth,
    TwoHundredFiftySixth,
}

#[allow(dead_code)]
pub enum Note {
    C,  // Do
    Db, // Re bemol
    D,  // Re
    Eb, // Mi bemol
    E,  // Mi
    F,  // Fa
    Gb, // Sol bemol
    G,  // Sol
    Ab, // La bemol
    A,  // La
    Bb, // Si bemol
    B,  // Si
}

pub struct Player {
    base_wait : u32,
    base_frequency : f32,
}

impl Value {
    pub fn as_millis(&self, base : u32) -> u64 {
        let edited = match self {
            Self::Maxima               => (base as u64) << 5,
            Self::Long                 => (base as u64) << 4,
            Self::DoubleWhole          => (base as u64) << 3,
            Self::Whole                => (base as u64) << 2,
            Self::Half                 => (base as u64) << 1,
            Self::Quarter              => (base as u64),
            Self::Eighth               => (base as u64) >> 1,
            Self::Sixteenth            => (base as u64) >> 2,
            Self::ThirtySecond         => (base as u64) >> 3,
            Self::SixtyFourth          => (base as u64) >> 4,
            Self::HundredTwentyEighth  => (base as u64) >> 5,
            Self::TwoHundredFiftySixth => (base as u64) >> 6,
        };

        edited
    }
}

impl Note {
    // base_frequency : Frequency at A3 (440Hz)
    pub fn as_frequency(&self, base_frequency : f32, position : i8) -> f32 {
        let mul = {
            let val = (1 << (3-position).abs()) as f32;

            if position < 3 {
                1. / val
            } else {
                val
            }
        };

        base_frequency * mul * (match self {
            Self::C  => 0.5946036,
            Self::Db => 0.6299605,
            Self::D  => 0.6674199,
            Self::Eb => 0.7071068,
            Self::E  => 0.7491535,
            Self::F  => 0.7937005,
            Self::Gb => 0.8408964,
            Self::G  => 0.8908987,
            Self::Ab => 0.9438743,
            Self::A  => 1.,
            Self::Bb => 1.059463,
            Self::B  => 1.122462,
        })
    }
}

impl Player {
    pub fn new(tempo : f32) -> Self {
        Self {
            base_wait: (60_000_000. / tempo) as u32,
            base_frequency: 440.,
        }
    }

    pub fn play_note(&self, value : Value, note : Note, scale : i8, continuous: bool) {
        let frequency = note.as_frequency(self.base_frequency, scale);
        let sleep_duration = value.as_millis(self.base_wait) - if !continuous { 50_000 } else { 0 };

        crate::play_sound(frequency);

        crate::sleep(sleep_duration);

        if !continuous {
            crate::stop_sound();
            crate::sleep(50_000);
        }
    }

    pub fn play_pause(&self, value : Value) {
        crate::stop_sound();
        crate::sleep(value.as_millis(self.base_wait));
    }
}
