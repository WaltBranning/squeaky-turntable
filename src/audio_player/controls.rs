use leptos::*;
use html::Audio;
// use leptos_dom::helpers::AnimationFrameRequest;
use leptos_icons::*;
use icondata::{
    IoPauseSharp, IoPlayBackSharp, IoPlayForwardSharp, IoPlaySharp, IoPlaySkipBackSharp,
    IoPlaySkipForwardSharp,
};

use logging::log;
use wasm_bindgen_futures::{self, JsFuture};
use crate::audio_player::{PlayerState, PlayerButton, PlayerStateSignal, TrackChangeSignals};

#[component]
pub fn Controls(
    audio_ref: NodeRef<Audio>, 
    play_state: PlayerStateSignal,
    track_change_signals: TrackChangeSignals
) -> impl IntoView {
    
    let playState = play_state.playing_state;
    let setPlayState = play_state.set_playing_state;
    // let (playState, setPlayState) = create_signal(PlayerState::Paused);
    let (ctrlBtnAction, setCtrlBtnAction) = create_signal(PlayerButton::Pause);
    let set_track = track_change_signals.set;

    create_effect(move |_| {
        let audio_ref_element = audio_ref.get().unwrap();
        // let curr_time = audio_ref_element.current_time();
        
        // log!("The PlayState is {:?}",playState());
        match ctrlBtnAction() {
            PlayerButton::Play => {
                let play_promise = JsFuture::from(audio_ref_element.play().unwrap());
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = play_promise.await;
                });
                setPlayState(PlayerState::Playing);
                audio_ref().unwrap().set_autoplay(true);
            }
            PlayerButton::FastForward => {
                let curr_time = audio_ref_element.current_time();
                audio_ref_element.set_current_time(curr_time + 10.);
            },
            PlayerButton::Rewind => {
                let curr_time = audio_ref_element.current_time();
                audio_ref_element.set_current_time(curr_time - 10.);
            },
            PlayerButton::Next => {
                set_track(Some(PlayerButton::Next));
            },
            PlayerButton::Previous => {
                let curr_time = audio_ref_element.current_time();
                if  curr_time > 5. {
                    audio_ref_element.set_current_time(0.);
                } else {
                    set_track(Some(PlayerButton::Previous));
                }
            },
            PlayerButton::Pause => {
                audio_ref_element.pause().unwrap();
                setPlayState(PlayerState::Paused);
                audio_ref().unwrap().set_autoplay(false);
                }
        }
    });

    view! {
        <div class="controls-wrapper flex flex-row align-content-center justify-content-center">
                <ControlButton control_type=move || PlayerButton::Previous 
                    on:click=move |_| {
                        setCtrlBtnAction(PlayerButton::Previous)
                    }
                />
                <ControlButton control_type=move || PlayerButton::Rewind 
                    on:click=move |_| {
                        setCtrlBtnAction(PlayerButton::Rewind)
                    }
                />
                <ControlButton control_type=move ||{
                    if let PlayerState::Playing = playState() {
                        log!("Playing");
                        PlayerButton::Play
                    } else {
                        log!("Paused");
                        PlayerButton::Pause
                    }
                }
                    on:click=move |_| {
                        if let PlayerState::Paused = playState() {
                            setCtrlBtnAction(PlayerButton::Play);
                        } else {
                            setCtrlBtnAction(PlayerButton::Pause);
                        }
                    }
                />
                <ControlButton control_type=move || PlayerButton::FastForward 
                    on:click=move |_| {
                        setCtrlBtnAction(PlayerButton::FastForward)
                    }
                />
                <ControlButton control_type=move || PlayerButton::Next 
                    on:click=move |_| {
                        setCtrlBtnAction(PlayerButton::Next)
                    }            
                />
        </div>
    }
}

#[component]
pub fn ControlButton(control_type: impl Fn() -> PlayerButton + 'static) -> impl IntoView {
    let player_state = MaybeSignal::derive(move || match control_type() {
        PlayerButton::Play => IoPauseSharp,
        PlayerButton::FastForward => IoPlayForwardSharp,
        PlayerButton::Next => IoPlaySkipForwardSharp,
        PlayerButton::Previous => IoPlaySkipBackSharp,
        PlayerButton::Rewind => IoPlayBackSharp,
        PlayerButton::Pause => IoPlaySharp,
    });
    view! {
        <div class="control-button inline-block">
            <Icon icon=player_state class="control-button-icon"/>
        </div>
    }
}