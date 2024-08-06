use leptos::*;
mod audio_player;
use audio_player::audio_player::AudioPlayer;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}


#[component]
fn App() -> impl IntoView {
    view! {
        <AudioPlayer />
    }
}




