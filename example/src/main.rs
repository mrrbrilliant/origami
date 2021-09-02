use origami::{WebView, WebViewSettings, Window};

fn main() {
    let window = Window::new_with_type("top_level");
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
    window.run()
}
