

use ev::Event;
use html::{Audio, Input};
use leptos::*;
use logging::log;
use web_sys::js_sys::JSON::parse;


use crate::audio_player::controls::Controls;
use crate::audio_player::Track;

#[component]
pub fn AudioPlayer() -> impl IntoView {
    let (current_track, set_current_track) = create_signal(Track::new());
    let audio_ref = create_node_ref::<Audio>();
    let progress_bar_ref = create_node_ref::<Input>();
    // let progress_bar_ref = 

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
                <DisplayTrack track=current_track audio_ref=audio_ref progress_bar_ref=progress_bar_ref/>
                <Controls  audio_ref=audio_ref progress_bar_ref=progress_bar_ref />
            </div>
        </div>
    }
}

#[component]
fn DisplayTrack(track: ReadSignal<Track>, audio_ref: NodeRef<Audio>, progress_bar_ref: NodeRef<Input>) -> impl IntoView {
    
    let (duration, setDuration) = create_signal(0.0);
    let (progress, setProgress) = create_signal(0.0);

    view! {
        <div class="display-track-wrapper">
            <audio 
                src=move ||{track().src} 
                _ref=audio_ref 
                on:loadedmetadata=move |_: Event| {setDuration(audio_ref.get().unwrap().duration())}
                set_current_time=progress
            />
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
                <TrackProgress 
                    audio_ref=audio_ref 
                    progress_ref=progress_bar_ref
                    duration=duration
                    set_progress=setProgress
                    progress=progress
                />
                <div class="display-track-labels">
                    <span class="title">{move || track().title}</span> -
                    <span class="artist">{move || track().artist}</span>
                </div>
            </div>
        </div>
    }
}



#[component]
fn TrackProgress(
    audio_ref: NodeRef<Audio>, 
    progress_ref: NodeRef<Input>, 
    duration: ReadSignal<f64>,
    set_progress: WriteSignal<f64>,
    progress: ReadSignal<f64>) -> impl IntoView {

    let audio_ref_element = audio_ref.get().unwrap();
    let current_time = move || audio_ref.get().unwrap().current_time(); 
    log!("{:?}",current_time());
    let handle_progress_change = move |ev:Event| {    
        audio_ref_element.set_current_time(event_target_value(&ev).parse::<f64>().unwrap_or_default());
        set_progress(event_target_value(&ev).parse::<f64>().unwrap_or_default());
    };

    view! {
        <div class="progress-bar-component-wrapper">
            <span class="pb-time_current">{current_time}</span>
            <input 
                type="range" 
                value="0"
                prop:max=duration
                node_ref=progress_ref
                on:input=handle_progress_change
            />
            <span class="pb-duration">{move || float_to_seconds(duration()-progress())}</span>
        </div>
    }
}

fn float_to_seconds(seconds_float: f64) -> String {
    let minutes = seconds_float / 60.;
    let seconds = seconds_float % 60.;
    format!("{:0>2.0}:{:0>2.0}", minutes, seconds)
}