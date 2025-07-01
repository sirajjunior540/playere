use regex::Regex;
use std::env;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
    webview::WebViewBuilder,
};

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

fn main() -> wry::Result<()> {
    let args: Vec<String> = env::args().collect();
    let initial_url = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("YouTube Player")
        .with_decorations(false)
        .with_always_on_top(true)
        .with_inner_size(wry::application::dpi::LogicalSize::new(1280, 720))
        .build(&event_loop)?;

    let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            background: #000;
            overflow: hidden;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
            user-select: none;
        }}
        #player-container {{
            width: 100vw;
            height: 100vh;
            position: relative;
        }}
        #youtube-player {{
            width: 100%;
            height: 100%;
            border: none;
        }}
        #drag-region {{
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 40px;
            -webkit-app-region: drag;
            app-region: drag;
            z-index: 999;
        }}
        #controls {{
            position: absolute;
            top: 10px;
            right: 10px;
            opacity: 0;
            transition: opacity 0.3s ease;
            z-index: 1000;
            -webkit-app-region: no-drag;
            app-region: no-drag;
        }}
        #controls:hover,
        #drag-region:hover ~ #controls {{
            opacity: 1;
        }}
        .control-btn {{
            background: rgba(0, 0, 0, 0.7);
            border: none;
            color: white;
            padding: 8px 12px;
            margin: 0 2px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
            transition: background 0.2s ease;
        }}
        .control-btn:hover {{
            background: rgba(0, 0, 0, 0.9);
        }}
        #url-input-container {{
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            background: rgba(0, 0, 0, 0.8);
            padding: 20px;
            border-radius: 8px;
            text-align: center;
            color: white;
        }}
        #url-input {{
            width: 400px;
            padding: 10px;
            margin: 10px 0;
            border: 1px solid #333;
            border-radius: 4px;
            background: #222;
            color: white;
            font-size: 14px;
        }}
        #load-btn {{
            background: #ff0000;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 4px;
            cursor: pointer;
            margin-left: 10px;
        }}
        #load-btn:hover {{
            background: #cc0000;
        }}
        .hidden {{
            display: none;
        }}
    </style>
</head>
<body>
    <div id="player-container">
        <iframe id="youtube-player" class="hidden" allowfullscreen 
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                referrerpolicy="strict-origin-when-cross-origin"></iframe>
        
        <div id="drag-region"></div>
        
        <div id="controls">
            <button class="control-btn" onclick="window.close()">✕</button>
        </div>
        
        <div id="url-input-container">
            <h2>YouTube Player</h2>
            <p>Enter a YouTube URL to start playing:</p>
            <div>
                <input type="text" id="url-input" placeholder="https://www.youtube.com/watch?v=..." />
                <button id="load-btn" onclick="loadVideo()">Load Video</button>
            </div>
        </div>
    </div>

    <script>
        function extractVideoId(url) {{
            const patterns = [
                /(?:youtube\.com\/watch\?v=|youtu\.be\/)([a-zA-Z0-9_-]{{11}})/,
                /youtube\.com\/embed\/([a-zA-Z0-9_-]{{11}})/,
                /youtube\.com\/v\/([a-zA-Z0-9_-]{{11}})/
            ];
            
            for (const pattern of patterns) {{
                const match = url.match(pattern);
                if (match && match[1]) {{
                    return match[1];
                }}
            }}
            return null;
        }}
        
        function createEmbedUrl(videoId) {{
            return `https://www.youtube.com/embed/${{videoId}}?autoplay=1&controls=1&modestbranding=1&rel=0&showinfo=0`;
        }}
        
        function loadVideo() {{
            const urlInput = document.getElementById('url-input');
            const url = urlInput.value.trim();
            
            if (!url) {{
                alert('Please enter a YouTube URL');
                return;
            }}
            
            const videoId = extractVideoId(url);
            if (!videoId) {{
                alert('Invalid YouTube URL');
                return;
            }}
            
            const playerElement = document.getElementById('youtube-player');
            const inputContainer = document.getElementById('url-input-container');
            
            playerElement.src = createEmbedUrl(videoId);
            playerElement.classList.remove('hidden');
            inputContainer.classList.add('hidden');
        }}
        
        document.getElementById('url-input').addEventListener('keypress', function(e) {{
            if (e.key === 'Enter') {{
                loadVideo();
            }}
        }});
        
        document.addEventListener('keydown', function(e) {{
            if (e.key === 'Escape') {{
                window.close();
            }}
        }});
        
        window.addEventListener('contextmenu', e => e.preventDefault());
        
        // Load initial URL if provided
        const initialUrl = "{}";
        if (initialUrl) {{
            document.getElementById('url-input').value = initialUrl;
            loadVideo();
        }}
    </script>
</body>
</html>
"#, initial_url.replace("\\", ""));

    let _webview = WebViewBuilder::new(window)?
        .with_html(html)?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("YouTube Player started"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}