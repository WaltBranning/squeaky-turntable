use leptos::{ReadSignal, WriteSignal};

pub mod audio_player;
pub mod controls;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerState {
    Playing,
    Paused,
}

pub struct PlayerStateSignal {
    playing_state: ReadSignal<PlayerState>,
    set_playing_state: WriteSignal<PlayerState>
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
    album: Option<String>,
    src: Option<String>,
    artist: Option<String>,
    img: Option<String>,
}

impl Track {
    pub fn new() -> Self {
        Self {
            index: None,
            title: None,
            album: None,
            src: None,
            artist: None,
            img: None,
        }
    }
}