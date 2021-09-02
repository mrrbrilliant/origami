use webkit2gtk::traits::{SettingsExt, UserContentManagerExt, WebContextExt, WebViewExt};
use webkit2gtk::{
    self, LoadEvent, Settings, UserContentManager, WebContext, WebView, WebViewExtManual,
};

pub struct OrigamiWebView {
    pub webview: WebView,
}

impl Default for OrigamiWebView {
    fn default() -> Self {
        let context = WebContext::default().unwrap();
        let user_content_manager = UserContentManager::new();
        let webview =
            WebView::new_with_context_and_user_content_manager(&context, &user_content_manager);

        Self { webview }
    }
}

impl OrigamiWebView {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn load_uri(&self, uri: &str) {
        self.webview.load_uri(uri);
    }
    pub fn settings(&self, configs: Vec<OrigamiWebViewSettings>) {
        let settings = WebViewExt::settings(&self.webview).unwrap();
        for config in configs.iter() {
            match config {
                OrigamiWebViewSettings::Accelerated2dCanvas(s) => {
                    settings.set_enable_accelerated_2d_canvas(*s)
                }
                OrigamiWebViewSettings::BackForwardNavigationGestures(s) => {
                    settings.set_enable_back_forward_navigation_gestures(*s)
                }
                OrigamiWebViewSettings::CaretBrowsing(s) => settings.set_enable_caret_browsing(*s),
                OrigamiWebViewSettings::DeveloperExtras(s) => {
                    settings.set_enable_developer_extras(*s)
                }
                OrigamiWebViewSettings::DnsPrefetching(s) => {
                    settings.set_enable_dns_prefetching(*s)
                }
                OrigamiWebViewSettings::EncryptedMedia(s) => {
                    settings.set_enable_encrypted_media(*s)
                }
                OrigamiWebViewSettings::FrameFlattening(s) => {
                    settings.set_enable_frame_flattening(*s)
                }
                OrigamiWebViewSettings::Fullscreen(s) => settings.set_enable_fullscreen(*s),
                OrigamiWebViewSettings::Html5Database(s) => settings.set_enable_html5_database(*s),
                OrigamiWebViewSettings::Html5LocalStorage(s) => {
                    settings.set_enable_html5_local_storage(*s)
                }
                OrigamiWebViewSettings::HyperlinkAuditing(s) => {
                    settings.set_enable_hyperlink_auditing(*s)
                }
                OrigamiWebViewSettings::Java(s) => settings.set_enable_java(*s),
                OrigamiWebViewSettings::Javascript(s) => settings.set_enable_javascript(*s),
                OrigamiWebViewSettings::JavascriptMarkup(s) => {
                    settings.set_enable_javascript_markup(*s)
                }
                OrigamiWebViewSettings::Media(s) => settings.set_enable_media(*s),
                OrigamiWebViewSettings::MediaCapabilities(s) => {
                    settings.set_enable_media_capabilities(*s)
                }
                OrigamiWebViewSettings::MediaStream(s) => settings.set_enable_media_stream(*s),
                OrigamiWebViewSettings::Mediasource(s) => settings.set_enable_mediasource(*s),
                OrigamiWebViewSettings::MockCaptureDevices(s) => {
                    settings.set_enable_mock_capture_devices(*s)
                }
                OrigamiWebViewSettings::OfflineWebApplicationCache(s) => {
                    settings.set_enable_offline_web_application_cache(*s)
                }
                OrigamiWebViewSettings::PageCache(s) => settings.set_enable_page_cache(*s),
                OrigamiWebViewSettings::Plugins(s) => settings.set_enable_plugins(*s),
                OrigamiWebViewSettings::PrivateBrowsing(s) => {
                    settings.set_enable_private_browsing(*s)
                }
                OrigamiWebViewSettings::ResizableTextAreas(s) => {
                    settings.set_enable_resizable_text_areas(*s)
                }
                OrigamiWebViewSettings::SiteSpecificQuirks(s) => {
                    settings.set_enable_site_specific_quirks(*s)
                }
                OrigamiWebViewSettings::SmoothScrolling(s) => {
                    settings.set_enable_smooth_scrolling(*s)
                }
                OrigamiWebViewSettings::SpatialNavigation(s) => {
                    settings.set_enable_spatial_navigation(*s)
                }
                OrigamiWebViewSettings::TabsToLinks(s) => settings.set_enable_tabs_to_links(*s),
                OrigamiWebViewSettings::Webaudio(s) => settings.set_enable_webaudio(*s),
                OrigamiWebViewSettings::Webgl(s) => settings.set_enable_webgl(*s),
                OrigamiWebViewSettings::WriteConsoleMessagesToStdout(s) => {
                    settings.set_enable_write_console_messages_to_stdout(*s)
                }
                OrigamiWebViewSettings::XssAuditor(s) => settings.set_enable_xss_auditor(*s),
            }
        }
    }
}

pub enum OrigamiWebViewSettings {
    Accelerated2dCanvas(bool),
    BackForwardNavigationGestures(bool),
    CaretBrowsing(bool),
    DeveloperExtras(bool),
    DnsPrefetching(bool),
    EncryptedMedia(bool),
    FrameFlattening(bool),
    Fullscreen(bool),
    Html5Database(bool),
    Html5LocalStorage(bool),
    HyperlinkAuditing(bool),
    Java(bool),
    Javascript(bool),
    JavascriptMarkup(bool),
    Media(bool),
    MediaCapabilities(bool),
    MediaStream(bool),
    Mediasource(bool),
    MockCaptureDevices(bool),
    OfflineWebApplicationCache(bool),
    PageCache(bool),
    Plugins(bool),
    PrivateBrowsing(bool),
    ResizableTextAreas(bool),
    SiteSpecificQuirks(bool),
    SmoothScrolling(bool),
    SpatialNavigation(bool),
    TabsToLinks(bool),
    Webaudio(bool),
    Webgl(bool),
    WriteConsoleMessagesToStdout(bool),
    XssAuditor(bool),
}
