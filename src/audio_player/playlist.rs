use html::div;
use leptos::*;
use logging::log;
use crate::audio_player::{AudioList, PlayerButton, Track, TrackChangeSignals, TrackSetter};


#[component]
pub fn PlayList(
    playlist_input: AudioList, 
    track_change_signals: TrackChangeSignals,
    track_setter: TrackSetter
) -> impl IntoView {
    let track_signal = track_change_signals.read;
    let _clear_track_signal = track_change_signals.set;
    let current_track = track_setter.read;
    let set_current_track = track_setter.set;
    // let (playList, setPlaylist) = create_signal(view!{});
    let (trackIndex, setTrackIndex) = create_signal(0 as usize);

    let playList = playlist_input.clone();
    let playList_views  = playList.clone().into_iter().enumerate().map(|(idx, track)| {

        let new_track = track.clone();
        let comp_track = track.clone();
        let title = track.title;
        let artist = track.artist;
        let _album = track.album;

        create_effect(move |_| {trackIndex();});

        view!{
            <div class="playlist-item-item" id=move|| {if comp_track == current_track.get(){"selected"} else {"not-selected"}}
                 on:click=move |_| {
                    setTrackIndex(idx);
                    set_current_track(new_track.clone());
                    log!("New {:?} \n Current {:?}", new_track, current_track.get());
                }
                
                >{title}<br/><span class="weak-text">By:</span>  {artist}
                
                <span></span>
            </div>
            
        }
    }).collect_view();

    create_effect(move |_|{
        let track_index = trackIndex();
        if current_track().src == None {set_current_track(playList[0].clone())};

        if let Some(PlayerButton::Next) = track_signal() {
            if trackIndex() < playlist_input.len()-1 {
                let new_index = track_index + 1;
                setTrackIndex(new_index);
                set_current_track(playList[new_index].clone());
             }
        } else if let Some(PlayerButton::Previous) = track_signal() {
            if trackIndex() > 0 {
                let new_index = track_index - 1;
                setTrackIndex(new_index);
                set_current_track(playList[new_index].clone());
             }
        }    
    });

    view! {
        <div class="text-center playlist-header">Playlist</div>
        <div class="playlist-item-wrapper flex flex-column">
        {playList_views}
        </div>
    }
}