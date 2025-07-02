use std::env;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

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
        .with_resizable(true)
        .with_inner_size(wry::application::dpi::LogicalSize::new(960, 540))
        .with_min_inner_size(wry::application::dpi::LogicalSize::new(480, 270))
        .build(&event_loop)?;

    let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>YouTube Player</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            background: #0a0a0a;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            color: #ffffff;
            overflow: hidden;
            user-select: none;
        }}
        
        .app-container {{
            width: 100vw;
            height: 100vh;
            position: relative;
            display: flex;
            flex-direction: column;
        }}
        
        .titlebar {{
            height: 32px;
            background: rgba(0, 0, 0, 0.8);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 12px;
            backdrop-filter: blur(10px);
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            cursor: default;
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            z-index: 1000;
            opacity: 0;
            transform: translateY(-100%);
            transition: all 0.3s ease;
        }}
        
        .app-container:hover .titlebar,
        .titlebar:hover,
        .titlebar.show {{
            opacity: 1;
            transform: translateY(0);
        }}
        
        .titlebar.draggable {{
            background: rgba(59, 130, 246, 0.2);
            cursor: move;
        }}
        
        .titlebar .title {{
            font-size: 13px;
            font-weight: 500;
            color: rgba(255, 255, 255, 0.8);
            min-width: 100px;
        }}
        
        .titlebar-paste {{
            display: flex;
            align-items: center;
            gap: 6px;
            flex: 1;
            max-width: 300px;
            -webkit-app-region: no-drag;
        }}
        
        .titlebar-input {{
            flex: 1;
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 4px;
            color: rgba(255, 255, 255, 0.9);
            font-size: 12px;
            padding: 4px 8px;
            height: 22px;
            outline: none;
            transition: all 0.2s ease;
            font-family: inherit;
        }}
        
        .titlebar-input:focus {{
            background: rgba(255, 255, 255, 0.15);
            border-color: rgba(255, 255, 255, 0.3);
            box-shadow: 0 0 0 1px rgba(255, 255, 255, 0.1);
        }}
        
        .titlebar-input::placeholder {{
            color: rgba(255, 255, 255, 0.4);
            font-size: 11px;
        }}
        
        .paste-quick-btn {{
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 4px;
            color: rgba(255, 255, 255, 0.6);
            cursor: pointer;
            font-size: 10px;
            padding: 4px 6px;
            height: 22px;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.2s ease;
            min-width: 22px;
        }}
        
        .paste-quick-btn:hover {{
            background: rgba(255, 255, 255, 0.15);
            border-color: rgba(255, 255, 255, 0.2);
            color: rgba(255, 255, 255, 0.9);
        }}
        
        .controls {{
            display: flex;
            gap: 8px;
            -webkit-app-region: no-drag;
        }}
        
        .control-btn {{
            width: 24px;
            height: 24px;
            border-radius: 4px;
            border: 1px solid rgba(255, 255, 255, 0.1);
            background: rgba(255, 255, 255, 0.05);
            color: rgba(255, 255, 255, 0.6);
            cursor: pointer;
            font-size: 11px;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.2s ease;
            backdrop-filter: blur(10px);
        }}
        
        .control-btn:hover {{
            background: rgba(255, 255, 255, 0.1);
            border-color: rgba(255, 255, 255, 0.2);
            color: rgba(255, 255, 255, 0.9);
        }}
        
        .control-btn.active {{
            background: rgba(255, 255, 255, 0.15);
            color: #ffffff;
            border-color: rgba(255, 255, 255, 0.3);
        }}
        
        .control-btn.close {{
            color: rgba(255, 100, 100, 0.8);
        }}
        
        .control-btn.close:hover {{
            background: rgba(255, 59, 48, 0.2);
            border-color: rgba(255, 59, 48, 0.3);
            color: #ff3b30;
        }}
        
        .main-content {{
            flex: 1;
            position: relative;
            background: #0a0a0a;
        }}
        
        .video-player {{
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            border: none;
            z-index: 1;
        }}
        
        .url-input-container {{
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            background: rgba(0, 0, 0, 0.9);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 12px;
            padding: 32px;
            min-width: 400px;
            text-align: center;
            backdrop-filter: blur(20px);
        }}
        
        .url-input-container h1 {{
            font-size: 24px;
            font-weight: 600;
            margin-bottom: 8px;
            color: #ffffff;
        }}
        
        .url-input-container p {{
            font-size: 14px;
            color: rgba(255, 255, 255, 0.7);
            margin-bottom: 24px;
        }}
        
        .input-group {{
            display: flex;
            flex-direction: column;
            gap: 16px;
        }}
        
        .url-input {{
            width: 100%;
            padding: 12px 16px;
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 8px;
            color: #ffffff;
            font-size: 14px;
            outline: none;
            transition: all 0.2s ease;
        }}
        
        .url-input:focus {{
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
        }}
        
        .url-input::placeholder {{
            color: rgba(255, 255, 255, 0.5);
        }}
        
        .button-group {{
            display: flex;
            gap: 12px;
        }}
        
        .btn {{
            flex: 1;
            padding: 12px 24px;
            border: none;
            border-radius: 8px;
            font-size: 14px;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.2s ease;
        }}
        
        .btn-primary {{
            background: #3b82f6;
            color: #ffffff;
        }}
        
        .btn-primary:hover {{
            background: #2563eb;
        }}
        
        .btn-secondary {{
            background: rgba(255, 255, 255, 0.1);
            color: #ffffff;
            border: 1px solid rgba(255, 255, 255, 0.2);
        }}
        
        .btn-secondary:hover {{
            background: rgba(255, 255, 255, 0.2);
        }}
        
        .hidden {{
            display: none;
        }}
        
        .loading {{
            opacity: 0.7;
            pointer-events: none;
        }}
        
        .hover-zone {{
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            height: 100px;
            z-index: 2;
            pointer-events: none;
        }}
        
        .iframe-overlay {{
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            z-index: 10;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.3s ease;
        }}
        
        .iframe-overlay.show {{
            opacity: 1;
        }}
        
        .more-videos-hint {{
            position: absolute;
            bottom: 80px;
            right: 20px;
            background: rgba(0, 0, 0, 0.8);
            color: white;
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 11px;
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.1);
            pointer-events: auto;
            cursor: pointer;
        }}
        
        .more-videos-hint:hover {{
            background: rgba(0, 0, 0, 0.9);
        }}
        
        @media (max-width: 480px) {{
            .url-input-container {{
                min-width: 320px;
                padding: 24px;
            }}
            
            .button-group {{
                flex-direction: column;
            }}
        }}
    </style>
</head>
<body>
    <div class="app-container">
        <div class="titlebar" id="titlebar">
            <div class="title">YouTube Player</div>
            <div class="titlebar-paste">
                <input type="text" 
                       id="titlebar-input" 
                       class="titlebar-input" 
                       placeholder="Paste URL..."
                       title="Type or paste YouTube URL here and press Enter"
                       autocomplete="off"
                       spellcheck="false">
                <button class="paste-quick-btn" id="paste-quick-btn" onclick="pasteQuick()" title="Paste URL from clipboard">
                    📋
                </button>
            </div>
            <div class="controls">
                <button class="control-btn" id="move-btn" onclick="toggleMove()" title="Move Mode">
                    <span id="move-icon">⊹</span>
                </button>
                <button class="control-btn active" id="pin-btn" onclick="togglePin()" title="Always on Top">
                    <span id="pin-icon">•</span>
                </button>
                <button class="control-btn" id="browser-btn" onclick="openInBrowser()" title="Open in Browser" style="display: none;">
                    <span id="browser-icon">↗</span>
                </button>
                <button class="control-btn close" onclick="closeApp()" title="Close">
                    ×
                </button>
            </div>
        </div>
        
        <div class="main-content">
            <iframe id="video-player" class="video-player hidden" 
                    allowfullscreen 
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share; fullscreen"
                    referrerpolicy="strict-origin-when-cross-origin">
            </iframe>
            
            <div id="iframe-overlay" class="iframe-overlay">
                <div class="more-videos-hint" onclick="showNavigationHelp()" title="Help with video navigation">
                    Right-click videos → Copy link → Paste here
                </div>
            </div>
            
            <div id="url-input-container" class="url-input-container">
                <h1>YouTube Player</h1>
                <p>Enter a YouTube video or playlist URL to start watching</p>
                
                <div class="input-group">
                    <input type="text" 
                           id="url-input" 
                           class="url-input" 
                           placeholder="https://www.youtube.com/watch?v=... or playlist"
                           title="Enter YouTube video or playlist URL"
                           autocomplete="off"
                           spellcheck="false">
                    
                    <div class="button-group">
                        <button id="paste-btn" class="btn btn-secondary" onclick="pasteFromClipboard()" title="Paste YouTube URL from clipboard">
                            📋 Paste
                        </button>
                        <button id="load-btn" class="btn btn-primary" onclick="loadVideo()" title="Load the entered video or playlist">
                            Load Video
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        const state = {{
            dragEnabled: false,
            pinned: true,
            currentVideoId: null,
            currentPlaylistId: null,
            watchingVideo: false,
            isPlaylist: false
        }};
        
        function showNavigationHelp() {{
            // Focus on titlebar input to encourage paste
            elements.titlebarInput.focus();
            elements.titlebarInput.placeholder = 'Right-click video → Copy link → Paste here';
            
            // Show titlebar if hidden
            showTitlebar();
            
            // Reset placeholder after a while
            setTimeout(() => {{
                elements.titlebarInput.placeholder = 'Paste URL...';
            }}, 5000);
        }}
        
        const elements = {{
            titlebar: document.getElementById('titlebar'),
            moveBtn: document.getElementById('move-btn'),
            moveIcon: document.getElementById('move-icon'),
            pinBtn: document.getElementById('pin-btn'),
            pinIcon: document.getElementById('pin-icon'),
            urlInput: document.getElementById('url-input'),
            urlContainer: document.getElementById('url-input-container'),
            videoPlayer: document.getElementById('video-player'),
            pasteBtn: document.getElementById('paste-btn'),
            loadBtn: document.getElementById('load-btn'),
            titlebarInput: document.getElementById('titlebar-input'),
            pasteQuickBtn: document.getElementById('paste-quick-btn'),
            iframeOverlay: document.getElementById('iframe-overlay')
        }};
        
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
        
        function extractPlaylistId(url) {{
            // More comprehensive playlist detection
            const patterns = [
                /[?&]list=([a-zA-Z0-9_-]+)/,
                /youtube\.com\/playlist\?list=([a-zA-Z0-9_-]+)/,
                /youtu\.be\/[^?]*\?.*list=([a-zA-Z0-9_-]+)/
            ];
            
            for (const pattern of patterns) {{
                const match = url.match(pattern);
                if (match && match[1]) {{
                    // Filter out invalid playlist IDs (like single video watch later, etc.)
                    const playlistId = match[1];
                    if (playlistId.length > 10 && !playlistId.startsWith('WL')) {{
                        return playlistId;
                    }}
                }}
            }}
            return null;
        }}
        
        function parseYouTubeUrl(url) {{
            try {{
                const videoId = extractVideoId(url);
                const playlistId = extractPlaylistId(url);
                
                // Debug logging
                console.log('Parsing URL:', url);
                console.log('Video ID:', videoId);
                console.log('Playlist ID:', playlistId);
                
                return {{
                    videoId,
                    playlistId,
                    isPlaylist: !!playlistId,
                    isVideo: !!videoId
                }};
            }} catch (error) {{
                console.log('Error parsing YouTube URL:', error);
                return {{
                    videoId: null,
                    playlistId: null,
                    isPlaylist: false,
                    isVideo: false
                }};
            }}
        }}
        
        function createEmbedUrl(params) {{
            if (typeof params === 'string') {{
                // Backward compatibility - single video ID
                return `https://www.youtube.com/embed/${{params}}?autoplay=1&controls=1&rel=1&fs=1&modestbranding=1&playsinline=1&enablejsapi=1&origin=${{window.location.origin}}`;
            }}
            
            const {{ videoId, playlistId, isPlaylist }} = params;
            let baseUrl = 'https://www.youtube.com/embed/';
            let queryParams = new URLSearchParams({{
                autoplay: '1',
                controls: '1',
                fs: '1',
                modestbranding: '1',
                playsinline: '1',
                enablejsapi: '1',
                origin: window.location.origin,
                // Enable navigation tracking
                widget_referrer: window.location.origin
            }});
            
            if (isPlaylist && playlistId) {{
                // For playlists, we can optionally start with a specific video
                if (videoId) {{
                    baseUrl += videoId;
                    queryParams.set('list', playlistId);
                }} else {{
                    baseUrl += `videoseries`;
                    queryParams.set('list', playlistId);
                }}
                // Enable related videos within the playlist
                queryParams.set('rel', '1');
            }} else if (videoId) {{
                baseUrl += videoId;
                // Enable related videos for better navigation
                queryParams.set('rel', '1');
            }}
            
            return `${{baseUrl}}?${{queryParams.toString()}}`;
        }}
        
        function showError(message) {{
            elements.urlInput.style.borderColor = '#ef4444';
            elements.urlInput.style.boxShadow = '0 0 0 3px rgba(239, 68, 68, 0.1)';
            elements.urlInput.focus();
            elements.urlInput.select();
            
            setTimeout(() => {{
                elements.urlInput.style.borderColor = 'rgba(255, 255, 255, 0.2)';
                elements.urlInput.style.boxShadow = 'none';
            }}, 2000);
        }}
        
        function loadVideoFromUrl(url, loadingElement, loadingText) {{
            if (!url) {{
                showError('Please enter a YouTube URL');
                return false;
            }}
            
            const urlData = parseYouTubeUrl(url);
            
            if (!urlData.isVideo && !urlData.isPlaylist) {{
                showError('Invalid YouTube URL or playlist');
                return false;
            }}
            
            if (loadingElement) {{
                loadingElement.classList.add('loading');
                if (loadingText) loadingElement.textContent = loadingText;
            }}
            
            if (urlData.isPlaylist) {{
                state.currentPlaylistId = urlData.playlistId;
                state.currentVideoId = urlData.videoId; // might be null
            }} else {{
                state.currentVideoId = urlData.videoId;
                state.currentPlaylistId = null;
            }}
            
            elements.videoPlayer.src = createEmbedUrl(urlData);
            
            setTimeout(() => {{
                elements.videoPlayer.classList.remove('hidden');
                elements.urlContainer.classList.add('hidden');
                if (loadingElement) {{
                    loadingElement.classList.remove('loading');
                    if (loadingText) {{
                        loadingElement.textContent = urlData.isPlaylist ? 'Load Playlist' : 'Load Video';
                    }}
                }}
                state.watchingVideo = true;
                state.isPlaylist = urlData.isPlaylist;
                
                // Show browser button when watching
                const browserBtn = document.getElementById('browser-btn');
                if (browserBtn) browserBtn.style.display = 'flex';
                
                // Show navigation overlay
                elements.iframeOverlay.classList.add('show');
                
                // Clear titlebar input
                elements.titlebarInput.value = '';
                
                // Start monitoring for navigation
                lastVideoId = state.currentVideoId;
                startNavigationMonitoring();
                
                // Set up iframe load event listener for navigation detection
                elements.videoPlayer.onload = () => {{
                    console.log('Iframe loaded, checking for navigation');
                    setTimeout(() => {{
                        try {{
                            const currentUrl = elements.videoPlayer.src;
                            const newVideoId = extractVideoId(currentUrl);
                            if (newVideoId && newVideoId !== lastVideoId) {{
                                console.log('Navigation detected via iframe load:', lastVideoId, '->', newVideoId);
                                handleVideoNavigation(newVideoId);
                            }}
                        }} catch (e) {{
                            console.log('Error checking navigation:', e);
                        }}
                    }}, 1000);
                }};
            }}, 500);
            
            return true;
        }}
        
        function loadVideo() {{
            const url = elements.urlInput.value.trim();
            loadVideoFromUrl(url, elements.loadBtn, 'Loading...');
        }}
        
        function toggleMove() {{
            state.dragEnabled = !state.dragEnabled;
            
            if (state.dragEnabled) {{
                elements.titlebar.classList.add('draggable');
                elements.moveBtn.classList.add('active');
                elements.moveIcon.textContent = '◉';
                elements.moveBtn.title = 'Drag Mode: ON';
            }} else {{
                elements.titlebar.classList.remove('draggable');
                elements.moveBtn.classList.remove('active');
                elements.moveIcon.textContent = '⊹';
                elements.moveBtn.title = 'Drag Mode: OFF';
            }}
        }}
        
        // Handle titlebar drag
        let isDragging = false;
        
        elements.titlebar.addEventListener('mousedown', (e) => {{
            if (state.dragEnabled && e.button === 0 && !e.target.closest('.controls')) {{
                isDragging = true;
                if (window.ipc) {{
                    window.ipc.postMessage('startDrag');
                }}
            }}
        }});
        
        elements.titlebar.addEventListener('mouseup', () => {{
            isDragging = false;
        }});
        
        elements.titlebar.addEventListener('mouseleave', () => {{
            isDragging = false;
        }});
        
        function togglePin() {{
            state.pinned = !state.pinned;
            
            if (state.pinned) {{
                elements.pinBtn.classList.add('active');
                elements.pinIcon.textContent = '•';
                elements.pinBtn.title = 'Always on Top: ON';
            }} else {{
                elements.pinBtn.classList.remove('active');
                elements.pinIcon.textContent = '○';
                elements.pinBtn.title = 'Always on Top: OFF';
            }}
            
            if (window.ipc) {{
                window.ipc.postMessage(`pin:${{state.pinned}}`);
            }}
        }}
        
        async function pasteFromClipboard() {{
            elements.pasteBtn.classList.add('loading');
            elements.pasteBtn.textContent = 'Pasting...';
            
            try {{
                console.log('Starting paste operation...');
                
                if (!navigator.clipboard) {{
                    throw new Error('Clipboard API not available');
                }}
                
                if (!navigator.clipboard.readText) {{
                    throw new Error('Clipboard readText not supported');
                }}
                
                console.log('Clipboard API available, reading text...');
                const text = await navigator.clipboard.readText();
                console.log('Clipboard text:', text);
                
                if (!text || !text.trim()) {{
                    throw new Error('Clipboard is empty');
                }}
                
                const trimmedText = text.trim();
                console.log('Trimmed text:', trimmedText);
                
                if (!trimmedText.includes('youtube.com') && !trimmedText.includes('youtu.be')) {{
                    throw new Error('No YouTube URL found in clipboard');
                }}
                
                console.log('YouTube URL detected, parsing...');
                const urlData = parseYouTubeUrl(trimmedText);
                console.log('URL data:', urlData);
                
                if (!urlData) {{
                    throw new Error('Failed to parse URL');
                }}
                
                if (!urlData.isVideo && !urlData.isPlaylist) {{
                    throw new Error('URL is not a valid YouTube video or playlist');
                }}
                
                console.log('URL is valid, setting input value...');
                elements.urlInput.value = trimmedText;
                elements.urlInput.focus();
                elements.urlInput.select();
                
                const contentType = urlData.isPlaylist ? 'Playlist' : 'Video';
                elements.pasteBtn.textContent = `✓ ${{contentType}} Pasted`;
                elements.pasteBtn.style.background = '#10b981';
                elements.pasteBtn.style.color = '#000';
                
                console.log('Paste successful!');
                
            }} catch (error) {{
                console.error('Paste failed:', error.message);
                elements.pasteBtn.textContent = 'Paste Failed';
                elements.pasteBtn.style.background = '#ef4444';
                elements.pasteBtn.style.color = '#fff';
                elements.urlInput.focus();
            }}
            
            setTimeout(() => {{
                elements.pasteBtn.classList.remove('loading');
                elements.pasteBtn.textContent = '📋 Paste';
                elements.pasteBtn.style.background = '';
                elements.pasteBtn.style.color = '';
            }}, 2000);
        }}
        
        async function pasteQuick() {{
            elements.pasteQuickBtn.style.opacity = '0.7';
            elements.pasteQuickBtn.textContent = '⟳';
            
            try {{
                console.log('Starting quick paste operation...');
                
                if (!navigator.clipboard || !navigator.clipboard.readText) {{
                    throw new Error('Clipboard API not supported');
                }}
                
                const text = await navigator.clipboard.readText();
                console.log('Quick paste clipboard text:', text);
                
                if (!text || !text.trim()) {{
                    throw new Error('Clipboard is empty');
                }}
                
                const trimmedText = text.trim();
                
                if (!trimmedText.includes('youtube.com') && !trimmedText.includes('youtu.be')) {{
                    throw new Error('No YouTube URL found in clipboard');
                }}
                
                const urlData = parseYouTubeUrl(trimmedText);
                console.log('Quick paste URL data:', urlData);
                
                if (!urlData || (!urlData.isVideo && !urlData.isPlaylist)) {{
                    throw new Error('Invalid YouTube URL format');
                }}
                
                // Load video directly without showing input container
                console.log('Loading video from quick paste...');
                loadVideoFromUrl(trimmedText);
                
                elements.pasteQuickBtn.textContent = '✓';
                elements.pasteQuickBtn.style.color = '#10b981';
                
            }} catch (error) {{
                console.error('Quick paste failed:', error.message);
                elements.pasteQuickBtn.textContent = '✗';
                elements.pasteQuickBtn.style.color = '#ef4444';
            }}
            
            setTimeout(() => {{
                elements.pasteQuickBtn.style.opacity = '1';
                elements.pasteQuickBtn.textContent = '📋';
                elements.pasteQuickBtn.style.color = '';
            }}, 1500);
        }}
        
        function closeApp() {{
            if (window.ipc) {{
                window.ipc.postMessage('close');
            }} else {{
                window.close();
            }}
        }}
        
        function showUrlInput() {{
            elements.videoPlayer.classList.add('hidden');
            elements.urlContainer.classList.remove('hidden');
            elements.urlInput.focus();
            elements.urlInput.select();
            state.watchingVideo = false;
            
            // Hide browser button when not watching
            const browserBtn = document.getElementById('browser-btn');
            if (browserBtn) browserBtn.style.display = 'none';
            
            // Hide navigation overlay
            elements.iframeOverlay.classList.remove('show');
            
            // Stop monitoring when not watching
            stopNavigationMonitoring();
        }}
        
        function openInBrowser() {{
            let url = '';
            
            if (state.isPlaylist && state.currentPlaylistId) {{
                url = `https://www.youtube.com/playlist?list=${{state.currentPlaylistId}}`;
                if (state.currentVideoId) {{
                    url = `https://www.youtube.com/watch?v=${{state.currentVideoId}}&list=${{state.currentPlaylistId}}`;
                }}
            }} else if (state.currentVideoId) {{
                url = `https://www.youtube.com/watch?v=${{state.currentVideoId}}`;
            }}
            
            if (url) {{
                window.open(url, '_blank');
            }}
        }}
        
        // Event listeners
        elements.urlInput.addEventListener('keydown', (e) => {{
            if (e.key === 'Enter') {{
                e.preventDefault();
                loadVideo();
            }}
        }});
        
        elements.titlebarInput.addEventListener('keydown', (e) => {{
            if (e.key === 'Enter') {{
                e.preventDefault();
                const url = elements.titlebarInput.value.trim();
                if (url) {{
                    loadVideoFromUrl(url);
                }}
            }}
        }});
        
        document.addEventListener('keydown', (e) => {{
            if (e.key === 'Escape') {{
                closeApp();
            }}
            
            if ((e.metaKey || e.ctrlKey) && e.key === 'v') {{
                if (!elements.urlContainer.classList.contains('hidden')) {{
                    return; // Let browser handle paste
                }}
                
                showUrlInput();
                e.preventDefault();
                pasteFromClipboard();
            }}
            
            if ((e.metaKey || e.ctrlKey) && e.key === 'n') {{
                e.preventDefault();
                showUrlInput();
            }}
        }});
        
        // Initialize
        window.addEventListener('load', () => {{
            elements.urlInput.focus();
            
            // Load initial URL if provided
            const initialUrl = "{}";
            if (initialUrl && initialUrl.trim()) {{
                elements.urlInput.value = initialUrl.trim();
                setTimeout(() => loadVideo(), 100);
            }}
        }});
        
        // Smart navigation detection system
        let lastVideoId = null;
        let navigationCheckInterval = null;
        let clickInterceptor = null;
        
        function startNavigationMonitoring() {{
            if (navigationCheckInterval) return;
            
            navigationCheckInterval = setInterval(() => {{
                try {{
                    const iframe = elements.videoPlayer;
                    if (!iframe.src) return;
                    
                    // Extract current video ID from iframe src
                    const currentVideoId = extractVideoId(iframe.src);
                    
                    if (currentVideoId && currentVideoId !== lastVideoId) {{
                        console.log('Video navigation detected:', lastVideoId, '->', currentVideoId);
                        
                        // Update our state
                        state.currentVideoId = currentVideoId;
                        lastVideoId = currentVideoId;
                        
                        // Clear titlebar input if user navigated via YouTube
                        if (elements.titlebarInput.value === '') {{
                            console.log('Auto-navigation detected, updating player state');
                        }}
                    }}
                }} catch (e) {{
                    // Ignore cross-origin errors
                }}
            }}, 1000);
        }}
        
        function stopNavigationMonitoring() {{
            if (navigationCheckInterval) {{
                clearInterval(navigationCheckInterval);
                navigationCheckInterval = null;
            }}
        }}
        
        
        // Listen for YouTube postMessage events
        window.addEventListener('message', (event) => {{
            if (event.origin === 'https://www.youtube.com') {{
                console.log('YouTube message:', event.data);
                
                // Handle YouTube API events
                if (event.data && typeof event.data === 'string') {{
                    try {{
                        const data = JSON.parse(event.data);
                        if (data.event === 'video-progress' && data.info) {{
                            // Video progress update
                            const videoData = data.info;
                            if (videoData.videoId && videoData.videoId !== state.currentVideoId) {{
                                console.log('Video changed via YouTube navigation');
                                handleVideoNavigation(videoData.videoId);
                            }}
                        }}
                    }} catch (e) {{
                        // Not JSON data, ignore
                    }}
                }}
            }}
        }});
        
        function handleVideoNavigation(newVideoId) {{
            console.log('Handling navigation to video:', newVideoId);
            
            // Prevent infinite loops
            if (newVideoId === state.currentVideoId || newVideoId === lastVideoId) {{
                return;
            }}
            
            // Update our state
            const oldVideoId = state.currentVideoId;
            state.currentVideoId = newVideoId;
            lastVideoId = newVideoId;
            
            // Create new embed URL for the new video
            const newUrl = createEmbedUrl({{
                videoId: newVideoId,
                playlistId: state.currentPlaylistId,
                isPlaylist: state.isPlaylist,
                isVideo: true
            }});
            
            // Temporarily stop monitoring to prevent loop
            stopNavigationMonitoring();
            
            // Update the iframe source
            elements.videoPlayer.src = newUrl;
            
            console.log(`🎥 Video Navigation: ${{oldVideoId}} -> ${{newVideoId}}`);
            
            // Restart monitoring after a delay
            setTimeout(() => {{
                startNavigationMonitoring();
            }}, 2000);
        }}
        
        // Handle titlebar visibility
        let hideTimeout;
        const showTitlebar = () => {{
            elements.titlebar.classList.add('show');
            clearTimeout(hideTimeout);
            hideTimeout = setTimeout(() => {{
                if (!isDragging) {{
                    elements.titlebar.classList.remove('show');
                }}
            }}, 3000);
        }};
        
        document.addEventListener('mousemove', (e) => {{
            if (e.clientY < 100) {{
                showTitlebar();
            }}
        }});
        
        // Keep titlebar visible when dragging
        elements.titlebar.addEventListener('mouseenter', showTitlebar);
        
        // Disable context menu
        window.addEventListener('contextmenu', e => e.preventDefault());
    </script>
</body>
</html>
"#, initial_url.replace("\\", ""));

    let _webview = WebViewBuilder::new(window)?
        .with_html(html)?
        .with_ipc_handler(move |window, message| {
            match message.as_str() {
                "close" => {
                    std::process::exit(0);
                }
                msg if msg.starts_with("pin:") => {
                    let pinned = msg.split(':').nth(1).unwrap_or("true") == "true";
                    window.set_always_on_top(pinned);
                }
                msg if msg.starts_with("startDrag") => {
                    if let Err(e) = window.drag_window() {
                        eprintln!("Failed to start drag: {:?}", e);
                    }
                }
                msg if msg.starts_with("navigate:") => {
                    println!("Navigation request: {}", msg);
                }
                _ => {}
            }
        })
        .with_navigation_handler(|_url| {
            // println!("Navigation to: {}", url);
            true // Allow all navigation
        })
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