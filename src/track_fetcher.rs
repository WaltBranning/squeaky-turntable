use crate::audio_player::{AudioList, Track};

pub fn get_files() -> AudioList {
    
    let file_list: AudioList = vec![

         Track {
            index: Some(1),
            title: Some("Chapter 1".to_string()),
            album: Some("Song of Songs".to_string()),
            src: Some("./audio/Song-Of-Songs/Song of Songs Chapter 1.mp3".to_string()),
            artist: Some("The Reformed Codex".to_string()),
            img: Some("./imgs/epic_song_of_songs.webp".to_string())
         },

         Track {
            index: Some(2),
            title: Some("Chapter 2".to_string()),
            album: Some("Song of Songs".to_string()),
            src: Some("./audio/Song-Of-Songs/Song of Songs Chapter 2.mp3".to_string()),
            artist: Some("The Reformed Codex".to_string()),
            img: Some("./imgs/epic_song_of_songs.webp".to_string())
         },

         Track {
            index: Some(3),
            title: Some("Chapter 3".to_string()),
            album: Some("The Reformed Codex".to_string()),
            src: Some("./audio/Song-Of-Songs/Song of Songs Chapter 3.mp3".to_string()),
            artist: Some("The Reformed Codex".to_string()),
            img: Some("./imgs/epic_song_of_songs.webp".to_string())
         },
        
         Track {
            index: Some(4),
            title: Some("Chapter 4".to_string()),
            album: Some("Song of Songs".to_string()),
            src: Some("./audio/Song-Of-Songs/Song of Songs Chapter 4.mp3".to_string()),
            artist: Some("The Reformed Codex".to_string()),
            img: Some("./imgs/epic_song_of_songs.webp".to_string())
         },

         Track {
            index: Some(5),
            title: Some("Chapter 5".to_string()),
            album: Some("Song of Songs".to_string()),
            src: Some("./audio/Song-Of-Songs/Song of Songs Chapter 5.mp3".to_string()),
            artist: Some("The Reformed Codex".to_string()),
            img: Some("./imgs/epic_song_of_songs.webp".to_string())
         },
        
         Track {
            index: Some(6),
            title: Some("Chapter 6".to_string()),
            album: Some("Song of Songs".to_string()),
            src: Some("./audio/Song-Of-Songs/Song of Songs Chapter 6.mp3".to_string()),
            artist: Some("The Reformed Codex".to_string()),
            img: Some("./imgs/epic_song_of_songs.webp".to_string())
         },
        
         Track {
            index: Some(7),
            title: Some("Chapter 7".to_string()),
            album: Some("Song of Songs".to_string()),
            src: Some("./audio/Song-Of-Songs/Song of Songs Chapter 7.mp3".to_string()),
            artist: Some("The Reformed Codex".to_string()),
            img: Some("./imgs/epic_song_of_songs.webp".to_string())
         },
         
         Track {
            index: Some(8),
            title: Some("Chapter 8".to_string()),
            album: Some("The Reformed Codex".to_string()),
            src: Some("./audio/Song-Of-Songs/Song of Songs Chapter 8.mp3".to_string()),
            artist: Some("The Reformed Codex".to_string()),
            img: Some("./imgs/epic_song_of_songs.webp".to_string())
         },
         
        
        Track {
           index: Some(9),
           title: Some("Chapter 1 - Alternate Version".to_string()),
           album: Some("Song of Songs".to_string()),
           src: Some("./audio/Song-Of-Songs/Song of Songs Chapter 1 Alternate Version.mp3".to_string()),
           artist: Some("The Reformed Codex".to_string()),
           img: Some("./imgs/epic_song_of_songs.webp".to_string())
        },
    ];

    file_list
}

