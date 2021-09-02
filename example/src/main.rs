use origami::{Request, Response, WebView, WebViewSettings, Window};

fn main() {
    let window = Window::default();
    let webview = WebView::new();
    webview.load_uri("https://youtube.com");

    #[cfg(debug_assertions)]
    webview.settings(vec![WebViewSettings::DeveloperExtras(true)]);

    webview.settings(vec![
        WebViewSettings::Accelerated2dCanvas(true),
        WebViewSettings::SmoothScrolling(true),
        WebViewSettings::Webaudio(true),
        WebViewSettings::Webgl(true),
        WebViewSettings::OfflineWebApplicationCache(true),
        WebViewSettings::PageCache(true),
    ]);

    window.bind(&webview);

    webview.ipc(|req| {
        let request = Request::from_str(&req);
        request.to_string()
    });

    window.run()
}
