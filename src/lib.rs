use std::error::Error;

pub use dark_light;
pub(crate) use egui::*;
pub(crate) use once_cell::sync::Lazy;

pub mod util;
pub(crate) use util::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
use windows as platform;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
use macos as platform;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
use linux as platform;

/// Caching whether the system is running dark mode or light mode so we don't have to detect it more then once.
pub(crate) static DARK_LIGHT_MODE: Lazy<dark_light::Mode> = Lazy::new(dark_light::detect);


pub fn system_theme() -> Result<Style, Box<dyn Error>> {
    let mut style = Style::default();
    style.visuals = match *DARK_LIGHT_MODE {
        dark_light::Mode::Default => Visuals::default(),
        dark_light::Mode::Dark => Visuals::dark(),
        dark_light::Mode::Light => Visuals::light(),
    };

    platform::style(&mut style)?;

    Ok(style)
}

pub trait VisualsExt {
    /// The color of the window titlebar when using system theme.
    fn titlebar(&self, focused: bool) -> Color32;
}
impl VisualsExt for Visuals {
    fn titlebar(&self, focused: bool) -> Color32 {
        if focused {
            self.widgets.noninteractive.weak_bg_fill
        } else {
            self.panel_fill
        }
    }
}