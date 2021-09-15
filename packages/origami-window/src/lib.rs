use gtk::prelude::*;
use gtk::traits::GtkWindowExt;
use origami_webview::OrigamiWebView;

pub struct OrigamiWindow {
    pub window: gtk::Window,
}

impl Default for OrigamiWindow {
    fn default() -> Self {
        gtk::init().unwrap();
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        Self { window }
    }
}

impl OrigamiWindow {
    pub fn new_with_type(window_type: &str) -> Self {
        gtk::init().unwrap();
        let window = match window_type {
            "top_level" => gtk::Window::new(gtk::WindowType::Toplevel),
            "popup" => gtk::Window::new(gtk::WindowType::Popup),
            _ => {
                eprintln!("Unknown Window Type");
                eprintln!("Using gtk::WindowType::Toplevel");
                gtk::Window::new(gtk::WindowType::Toplevel)
            }
        };

        Self { window }
    }

    pub fn default_size(&self, width: i32, height: i32) {
        self.window.set_default_size(width, height);
    }

    pub fn resize(&self, width: i32, height: i32) {
        self.window.resize(width, height);
    }

    pub fn resizable(&self, option: bool) {
        self.window.set_resizable(option);
    }

    pub fn frame(&self, option: bool) {
        self.window.set_decorated(option);
    }

    pub fn set_hint(&self, hint: &str) {
        match hint {
            "Normal" | "normal" => self.window.set_type_hint(gdk::WindowTypeHint::Normal),
            "Dialog" | "dialog" => self.window.set_type_hint(gdk::WindowTypeHint::Dialog),
            "Menu" | "menu" => self.window.set_type_hint(gdk::WindowTypeHint::Menu),
            "Toolbar" | "toolbar" => self.window.set_type_hint(gdk::WindowTypeHint::Toolbar),
            "Splashscreen" | "splashscreen" => {
                self.window.set_type_hint(gdk::WindowTypeHint::Splashscreen)
            }
            "Utility" | "utility" => self.window.set_type_hint(gdk::WindowTypeHint::Utility),
            "Dock" | "dock" => self.window.set_type_hint(gdk::WindowTypeHint::Dock),
            "Desktop" | "desktop" => self.window.set_type_hint(gdk::WindowTypeHint::Desktop),
            "DropdownMenu" | "dropdownmenu" => {
                self.window.set_type_hint(gdk::WindowTypeHint::DropdownMenu)
            }
            "PopupMenu" | "popupmenu" => self.window.set_type_hint(gdk::WindowTypeHint::PopupMenu),
            "Tooltip" | "tooltip" => self.window.set_type_hint(gdk::WindowTypeHint::Tooltip),
            "Notification" | "notification" => {
                self.window.set_type_hint(gdk::WindowTypeHint::Notification)
            }
            "Combo" | "comboa" => self.window.set_type_hint(gdk::WindowTypeHint::Combo),
            "Dnd" | "dnd" => self.window.set_type_hint(gdk::WindowTypeHint::Dnd),
            _ => {
                eprintln!("Invalid WindowTypeHint");
                eprintln!("Using WindowTypeHint::Normal");
                self.window.set_type_hint(gdk::WindowTypeHint::Normal)
            }
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.window.set_title(&title);
    }

    pub fn bind(&self, webview: &OrigamiWebView) {
        self.window.add(&webview.webview);
    }

    pub fn run(&self) {
        self.window.show_all();

        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });
        gtk::main();
    }
}
