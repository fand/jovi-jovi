#[derive(Debug, Clone, PartialEq)]
pub struct Voice {
    pub index: usize,
    pub id: &'static str,
    pub name: &'static str,
    pub filename: &'static str,
    pub is_playing: bool,
}

pub const VOICES: [Voice; 5] = [
    Voice {
        index: 0,
        id: "Joy",
        name: "JOY",
        filename: "sounds/joy.mp3",
        is_playing: false,
    },
    Voice {
        index: 1,
        id: "Divi",
        name: "DIVI",
        filename: "sounds/divi.mp3",
        is_playing: false,
    },
    Voice {
        index: 2,
        id: "John",
        name: "JOHN",
        filename: "sounds/john.mp3",
        is_playing: false,
    },
    Voice {
        index: 3,
        id: "Bon",
        name: "BON",
        filename: "sounds/bon.mp3",
        is_playing: false,
    },
    Voice {
        index: 4,
        id: "Jovi",
        name: "JOVI",
        filename: "sounds/jovi.mp3",
        is_playing: false,
    },
];
