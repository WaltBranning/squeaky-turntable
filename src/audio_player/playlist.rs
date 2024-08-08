use leptos::*;
use logging::log;
use crate::audio_player::{Track, AudioList, TrackChangeSignals, TrackSetter, PlayerButton};


#[component]
pub fn PlayList(
    playlist_input: AudioList, 
    track_change_signals: TrackChangeSignals,
    track_setter: TrackSetter
) -> impl IntoView {
    let track_signal = track_change_signals.read;
    let clear_track_signal = track_change_signals.set;
    let current_track = track_setter.read;
    let set_current_track = track_setter.set;
    let (playList, setPlaylist) = create_signal(view!{});
    let (trackIndex, setTrackIndex) = create_signal(0 as usize);

    let playList = playlist_input.clone();
    let playList_views  = playList.clone().into_iter().enumerate().map(|(idx, track)| {
        // if track.index
        view!{
            Hello
            {idx}
        }
    }).collect_view();

    create_effect(move |_|{
        let track_index = trackIndex();

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
    
        log!("{:?}", trackIndex());
    
    });

    view! {
        <div>{playList_views}</div>
    }
}