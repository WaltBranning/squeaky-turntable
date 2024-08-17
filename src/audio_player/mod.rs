use leptos::{ReadSignal, WriteSignal};

pub mod audio_player;
pub mod controls;
pub mod display_track;
pub mod playlist;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerState {
    Playing,
    Paused,
}

pub struct PlayerStateSignal {
    playing_state: ReadSignal<PlayerState>,
    set_playing_state: WriteSignal<PlayerState>
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerButton {
    Play,
    Pause,
    FastForward,
    Rewind,
    Next,
    Previous,
}

pub type AudioList = Vec<Track>;

pub struct TrackSetter {
    read: ReadSignal<Track>,
    set: WriteSignal<Track>
}

pub struct  TrackChangeSignals {
    read: ReadSignal<Option<PlayerButton>>,
    set: WriteSignal<Option<PlayerButton>>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    pub index: Option<usize>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub src: Option<String>,
    pub artist: Option<String>,
    pub img: Option<String>,
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