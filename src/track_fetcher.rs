use crate::audio_player::{AudioList, Track};

pub fn get_files() -> AudioList {
    
    let file_list: AudioList = vec![
        Track {
           index: Some(0),
           title: Some("Appalachian Muse".to_string()),
           album: Some("Appalachian Muse".to_string()),
           src: Some("./audio/Appalachian_Muse.mp3".to_string()),
           artist: Some("Appalachian Muse".to_string()),
           img: Some("./imgs/Appalachian_Muse.jpeg".to_string())
        },
        
        Track {
           index: Some(0),
           title: Some("Epic Rift Crossing".to_string()),
           album: Some("Cosmic Buccaneers".to_string()),
           src: Some("./audio/Cosmic_Buccaneers_Epic.mp3".to_string()),
           artist: Some("Cosmic Buccaneers".to_string()),
           img: Some("./imgs/CosmicBucaneers.jpeg".to_string())
        },
        
        Track {
           index: Some(0),
           title: Some("Cosmic Buccaneers Opening".to_string()),
           album: Some("Cosmic Buccaneers".to_string()),
           src: Some("./audio/Cosmic_Buccaneers_Opening.mp3".to_string()),
           artist: Some("Cosmic Buccaneers".to_string()),
           img: Some("./imgs/CosmicBucaneers.jpeg".to_string())
        },
        
        Track {
           index: Some(0),
           title: Some("Epic Song Of Songs Chapter One".to_string()),
           album: Some("Song of Songs - AI Renditions".to_string()),
           src: Some("./audio/Epic Song Of Songs Chapter One.mp3".to_string()),
           artist: Some("Song of Songs - AI Renditions".to_string()),
           img: Some("./imgs/Epic Song Of Songs Chapter One.jpeg".to_string())
        },
        
        Track {
           index: Some(0),
           title: Some("Piracy.mp3".to_string()),
           album: Some("Cosmic Buccaneers".to_string()),
           src: Some("./audio/Galactic_Piracy.mp3".to_string()),
           artist: Some("Cosmic Buccaneers".to_string()),
           img: Some("./imgs/CosmicBucaneers.jpeg".to_string())
        },
        
        Track {
           index: Some(0),
           title: Some("Extension.mp3".to_string()),
           album: Some("Cosmic Buccaneers".to_string()),
           src: Some("./audio/Extension.mp3".to_string()),
           artist: Some("Cosmic Buccaneers".to_string()),
           img: Some("./imgs/Extension.jpeg".to_string())
        },
    ];

    file_list
}

