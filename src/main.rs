use leptos::*;
mod audio_player;
use audio_player::audio_player::AudioPlayer;

mod track_fetcher;
use track_fetcher::*;

use icondata::{BsInfoCircle, SiRust, SiLeptos};
use leptos_icons::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let mp3s = get_files();
    view! {
        <div class="main-container">
            <div class="top-bar flex justify-content-between align-items-center">
                <span>
                    <span class="cinzel-400 top-bar-title">The</span>
                    <span class="fondamento-regular-italic" id="top-bar-emphasized">Reformed</span>
                    <span class="cinzel-400 top-bar-title">Codex</span></span>
                <Icon icon=BsInfoCircle />
                </div>
            <div class="center">
            <AudioPlayer track_list=mp3s/>
            </div>
            <div class="footer-icons">
                <a class="mr-2" href="https://www.rust-lang.org/"><Icon icon=SiRust /></a>
                <a href="https://leptos.dev/"><Icon icon=SiLeptos /></a>
                </div>
        </div>
    }
}




