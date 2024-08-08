use leptos::*;
mod audio_player;
use audio_player::audio_player::AudioPlayer;

mod track_fetcher;
use track_fetcher::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let mp3s = get_files();
    view! {
        <AudioPlayer track_list=mp3s/>
    }
}




