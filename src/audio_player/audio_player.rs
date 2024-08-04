

use html::Audio;
use leptos::*;


use crate::audio_player::controls::Controls;
use crate::audio_player::Track;

#[component]
pub fn AudioPlayer() -> impl IntoView {
    let (current_track, set_current_track) = create_signal(Track::new());
    let audio_ref = create_node_ref::<Audio>();

    let track = Track {
        index: Some(0),
        title: Some("Heaven in the New Testament".to_string()),
        album: Some("Metropolitan Tabernacle".to_string()),
        src: Some("https://samedia-b2-east.b-cdn.net/com-sermonaudio-sermons2/722414303985.mp3?ts=1719931005&bcdn_filename=2024-06-30+-+Heaven+in+the+New+Testament+-+Dr.+Peter+Masters+%28722414303985%29.mp3".to_string()),
        artist: Some("Dr. Peter Masters".to_string()),
        img: Some("/imgs/dr_peter_masters.jpg".to_string()),
    };

    set_current_track.set(track);
    view! {
        <div class="audio-player">
            <div class="inner">
                <DisplayTrack track=current_track audio_ref=audio_ref />
                <Controls  audio_ref=audio_ref />
            </div>
        </div>
    }
}

#[component]
fn DisplayTrack(track: ReadSignal<Track>, audio_ref: NodeRef<Audio>) -> impl IntoView {
    view! {
        <div class="display-track-wrapper">
            <audio src=move ||{track().src} _ref=audio_ref />
            <div class="audio-info" >
                <div class="display-track-image" >
                    <img src=move || {
                        if track().img != None {
                            track().img
                        } else {
                            Some("/imgs/music_placeholder.webp".to_string())
                        }
                    } />
                </div>
                <TrackProgress audio_ref=audio_ref />
                <div class="display-track-labels">
                    <span class="title">{move || track().title}</span> -
                    <span class="artist">{move || track().artist}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProgressBar(audio_ref: NodeRef<Audio>) -> impl IntoView {
    view! {
        <div class="progress-bar-component-wrapper">
            <span class="pb-time_current"></span>
            <input 
                type="range" 
                value="0"
            />
            <span class="pb-duration"></span>
        </div>
    }
}

#[component]
fn TrackProgress(audio_ref: NodeRef<Audio>) -> impl IntoView {
    let _ar = move || audio_ref();
    view! {
        <div class="track-progress-wrapper">
            <ProgressBar audio_ref=audio_ref />
        </div>
    }
}
