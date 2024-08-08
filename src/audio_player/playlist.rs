use leptos::*;
use logging::log;
use crate::audio_player::{Track, AudioList, TrackChangeSignals, TrackSetter};


#[component]
pub fn PlayList(
    playlist_input: AudioList, 
    track_change_signals: TrackChangeSignals,
    track_setter: TrackSetter
) -> impl IntoView {
    let track_signal = track_change_signals.read;
    let clear_track_signal = track_change_signals.set;
    let current_track = track_setter.read;
    let (playList, setPlaylist) = create_signal(view!{});


    let playList  = playlist_input.clone().into_iter().enumerate().map(|(idx, track)| {
        // if track.index
        view!{
            Hello
            {idx}
        }
    }).collect_view();

    create_effect(move |_|{
        log!("{:?}", current_track());
        for i in playlist_input.clone() {
            log!("{:?}", i);
        }
    });

    view! {
        <div>{playList}</div>
    }
}