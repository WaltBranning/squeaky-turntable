use leptos::*;
use ev::Event;
use html::Audio;
use leptos_use::{use_raf_fn_with_options, UseRafFnOptions};
use leptos_use::utils::Pausable;
use crate::audio_player::{Track, PlayerStateSignal, PlayerState};

#[component]
pub fn DisplayTrack(
    play_state: PlayerStateSignal,
    track: ReadSignal<Track>, 
    audio_ref: NodeRef<Audio>, 
    ) -> impl IntoView {
    
    let (duration, setDuration) = create_signal(0.0);
    
    view!{
        <div class="display-track-wrapper m-auto w-10">
            <audio 
                src=move ||{track().src} 
                _ref=audio_ref 
                on:loadedmetadata=move |_: Event| {setDuration(audio_ref.get().unwrap().duration())}
            />
            <div class="audio-info" >
                <div class="display-track-image inline-block " >
                    <img class="album-cover"
                        src=move || {
                        if track().img != None {
                            track().img
                        } else {
                            Some("/imgs/music_placeholder.webp".to_string())
                        }
                    } />
                </div>
                <TrackProgress 
                    audio_ref=audio_ref 
                    play_state=play_state
                    duration=duration
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
pub fn TrackProgress(
    audio_ref: NodeRef<Audio>, 
    play_state: PlayerStateSignal,
    duration: ReadSignal<f64>
    ) -> impl IntoView {

    let playState = play_state.playing_state;
    let (currentTime, setCurrentTime) = create_signal(0.);
    let handle_progress_change = move |ev:Event| {    
        audio_ref.get().unwrap()
            .set_current_time(event_target_value(&ev)
            .parse::<f64>()
            .unwrap_or_default());
    };

    
    let raf_option = UseRafFnOptions::default().immediate(false);
    let Pausable { pause, resume, is_active } = use_raf_fn_with_options(move |_| {
        let time = audio_ref.get().unwrap().current_time();
        setCurrentTime(time);
    }, raf_option);

    create_effect(move |_| {
        let _active = is_active();
        match playState.get() {
            PlayerState::Playing => resume(),
            PlayerState::Paused => pause()
        }
    });

    view! {
        <div class="progress-bar-component-wrapper flex justify-content-between align-content-center">
            <span class="time-display pb-time-current">{move || float_to_seconds(currentTime())}</span>
            <input class="w-11"
                type="range" 
                value="0"
                prop:max=duration
                prop:value=currentTime
                on:input=handle_progress_change
            />
            <span class="time-display pb-duration">{move || float_to_seconds(duration()-currentTime())}</span>
        </div>
    }
}

fn float_to_seconds(seconds_float: f64) -> String {
    let minutes = seconds_float / 60.;
    let seconds = seconds_float % 60.;
    format!("{:0>2.0}:{:0>2.0}", minutes, seconds)
} 