use leptos::*;
use html::Audio;
use leptos_icons::*;
use icondata::{
    IoPauseSharp, IoPlayBackSharp, IoPlayForwardSharp, IoPlaySharp, IoPlaySkipBackSharp,
    IoPlaySkipForwardSharp,
};

use logging::log;
use wasm_bindgen_futures::{self, JsFuture};
use crate::audio_player::{PlayerState, PlayerButton};

#[component]
pub fn Controls(audio_ref: NodeRef<Audio>) -> impl IntoView {
    let (playState, setPlayState) = create_signal(PlayerState::Paused);
    let (ctrlBtnAction, setCtrlBtnAction) = create_signal(PlayerButton::Pause);

    create_effect(move |_| {
        let audio_ref_element = audio_ref.get().unwrap();
        let curr_time = audio_ref_element.current_time();
        log!("The time is {}",curr_time);
        log!("The PlayState is {:?}",playState());
        match ctrlBtnAction() {
            PlayerButton::Play => {
                let play_promise = JsFuture::from(audio_ref_element.play().unwrap());
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = play_promise.await;
                });
                setPlayState(PlayerState::Playing);
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
                
            },
            PlayerButton::Previous => {
                let curr_time = audio_ref_element.current_time();
                if  curr_time > 5. {
                    audio_ref_element.set_current_time(0.);
                }
            },
            PlayerButton::Pause => {
                audio_ref_element.pause().unwrap();
                setPlayState(PlayerState::Paused);
            }
        }
    });

    view! {
        <div class="controls-wrapper">
            <div class="controls">
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
        <button>
            <Icon icon=player_state style="width: 1.75em; height: 1.75em" />
        </button>
    }
}