use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use tauri::{command, generate_context, generate_handler, Builder, Manager, Window};

#[derive(Serialize, Deserialize)]
struct YouTubePlayer {
    video_id: String,
    embed_url: String,
}

fn extract_video_id(url: &str) -> Option<String> {
    let patterns = vec![
        r"(?:youtube\.com/watch\?v=|youtu\.be/)([a-zA-Z0-9_-]{11})",
        r"youtube\.com/embed/([a-zA-Z0-9_-]{11})",
        r"youtube\.com/v/([a-zA-Z0-9_-]{11})",
    ];
    
    for pattern in patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(captures) = re.captures(url) {
                if let Some(video_id) = captures.get(1) {
                    return Some(video_id.as_str().to_string());
                }
            }
        }
    }
    None
}

fn create_embed_url(video_id: &str) -> String {
    format!(
        "https://www.youtube.com/embed/{}?autoplay=1&controls=1&modestbranding=1&rel=0&showinfo=0",
        video_id
    )
}

#[command]
fn load_video(url: String) -> Result<YouTubePlayer, String> {
    if let Some(video_id) = extract_video_id(&url) {
        let embed_url = create_embed_url(&video_id);
        Ok(YouTubePlayer { video_id, embed_url })
    } else {
        Err("Invalid YouTube URL".to_string())
    }
}

#[command]
fn close_app(window: Window) {
    window.close().unwrap();
}

#[command]
fn drag_window(window: Window) {
    window.start_dragging().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut initial_url = String::new();
    
    if args.len() > 1 {
        initial_url = args[1].clone();
    }

    Builder::default()
        .invoke_handler(generate_handler![load_video, close_app, drag_window])
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            
            if !initial_url.is_empty() {
                if let Some(video_id) = extract_video_id(&initial_url) {
                    let embed_url = create_embed_url(&video_id);
                    let player = YouTubePlayer { video_id, embed_url };
                    window.emit("load-video", &player).unwrap();
                }
            }
            
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running Tauri application");
}
