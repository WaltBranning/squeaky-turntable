

pub mod audio_player;
pub mod controls;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerState {
    Playing,
    Paused,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerButton {
    Play,
    Pause,
    FastForward,
    Rewind,
    Next,
    Previous,
}

#[derive(Clone)]
pub struct Track {
    index: Option<u8>,
    title: Option<String>,
    src: Option<String>,
    author: Option<String>,
    img: Option<String>,
}

impl Track {
    pub fn new() -> Self {
        Self {
            index: None,
            title: None,
            src: None,
            author: None,
            img: None,
        }
    }
}