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

#[component]
fn ProgressBar(progress: impl Fn() -> i32 + 'static , #[prop(default = 100)] max:u16) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}




