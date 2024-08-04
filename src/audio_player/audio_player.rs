

use html::Audio;
use leptos::*;
use leptos_icons::*;
use logging::log;
use wasm_bindgen_futures::{self, JsFuture};

use crate::audio_player::controls::*;
use crate::audio_player::{Track, PlayerState, PlayerButton};

#[component]
pub fn AudioPlayer() -> impl IntoView {
    let (current_track, set_current_track) = create_signal(Track::new());
    let audio_ref = create_node_ref::<Audio>();

    let track = Track {
        index: Some(0),
        title: Some("Heaven in the New Testament".to_string()),
        src: Some("https://samedia-b2-east.b-cdn.net/com-sermonaudio-sermons2/722414303985.mp3?ts=1719931005&bcdn_filename=2024-06-30+-+Heaven+in+the+New+Testament+-+Dr.+Peter+Masters+%28722414303985%29.mp3".to_string()),
        author: Some("Dr. Peter Masters".to_string()),
        img: Some("/static/imgs/dr_peter_masters.jpg".to_string()),
    };
    set_current_track.set(track);
    view! {
        <div class="audio-player">
            <div class="inner">
                <DisplayTrack track=current_track audio_ref=audio_ref />
                <TrackProgress audio_ref=audio_ref />
                <Controls  audio_ref=audio_ref />
                
            </div>
        </div>
    }
}

#[component]
fn DisplayTrack(track: ReadSignal<Track>, audio_ref: NodeRef<Audio>) -> impl IntoView {
    let image = move || track().img;
    view! {
        <div class="display-track-wrapper">
            <audio src=move ||{track().src} _ref=audio_ref />
            <div class="audio-info" >
                <div class="audio-image" >
                    <img src=move || track().img />
                </div>
            </div>
        </div>
    }
}

#[component]
fn TrackProgress(audio_ref: NodeRef<Audio>) -> impl IntoView {
    view! {
        <div class="track-progress-wrapper">
            Progress Bar!
        </div>
    }
}
