use leptos::*;
use html::Audio;

use crate::audio_player::controls::Controls;
use crate::audio_player::display_track::DisplayTrack;
use crate::audio_player::playlist::PlayList;
use crate::audio_player::{Track, PlayerStateSignal, PlayerState, AudioList, TrackChangeSignals, TrackSetter};


#[component]
pub fn AudioPlayer(track_list: AudioList) -> impl IntoView {
    let (playState, setPlayState) = create_signal(PlayerState::Paused);
    let (current_track, setCurrentTrack) = create_signal(Track::new());
    let (changeTrackSignal, setChangeTrackSignal) = create_signal(None);
    let audio_ref = create_node_ref::<Audio>();
    // let progress_bar_ref = create_node_ref::<Input>();

    // let track = track_list[0].clone();

    // setCurrentTrack.set(track);
    view! {
        <div class="player-container audio-player border-round align-items-center">
            <div class="player-interface">
                <DisplayTrack 
                    play_state=PlayerStateSignal{playing_state:playState, set_playing_state:setPlayState}
                    track=current_track 
                    audio_ref=audio_ref
                />
                <Controls  
                    audio_ref=audio_ref 
                    play_state=PlayerStateSignal{
                        playing_state:playState, 
                        set_playing_state:setPlayState
                    }
                    track_change_signals=TrackChangeSignals{
                        read: changeTrackSignal,
                        set: setChangeTrackSignal
                    }
                />
            </div>
            <div class="playlist-wrapper ">
                <PlayList 
                playlist_input=track_list 
                track_change_signals=TrackChangeSignals{
                    read: changeTrackSignal,
                    set: setChangeTrackSignal
                }
                track_setter=TrackSetter{
                    read: current_track,
                    set: setCurrentTrack
                }
                />
            </div>
        </div>
    }
}