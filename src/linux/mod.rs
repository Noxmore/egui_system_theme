use std::{env, error::Error, io, path::Path};

use configparser::ini::Ini;
use epaint::Shadow;

use crate::*;

mod gtk;

pub fn style(style: &mut Style) -> Result<(), Box<dyn Error>> {
    if env::var("XDG_CURRENT_DESKTOP") == Ok("KDE".to_string())
        || env::var("DESKTOP_SESSION") == Ok("plasma".to_string())
    {
        style_kde(style)?;
    } else {
        style_gtk(style, 4).or_else(|_| style_gtk(style, 3))?;
    }

    // DesktopEnvironment::Cinnamon => detect_gtk("/org/cinnamon/desktop/interface/gtk-theme"),
    // DesktopEnvironment::Gnome => detect_gtk("/org/gnome/desktop/interface/gtk-theme"),
    // DesktopEnvironment::Mate => detect_gtk("/org/mate/desktop/interface/gtk-theme"),
    // DesktopEnvironment::Unity => detect_gtk("/org/gnome/desktop/interface/gtk-theme"),

    Ok(())
}

#[rustfmt::skip] // I want these macro calls to stay in one line each
pub fn style_kde(style: &mut Style) -> Result<(), Box<dyn Error>> {
    // TODO fonts
    let mut kdeglobals = Ini::new();
    kdeglobals.load(Path::new(&env::var("HOME")?).join(".config/kdeglobals"))?;

    macro_rules! set_color {($path:expr, $section:expr, $key:expr) => {
        if let Ok(color) = kdeglobals.get_color($section, $key) {
            $path = color;
        }
    };}
    macro_rules! set_stroke {($path:expr, $section:expr, $key:expr, $width:expr) => {
        if let Ok(color) = kdeglobals.get_color($section, $key) {
            $path = Stroke::new($width, color);
        }
    };}

    set_color!(style.visuals.widgets.noninteractive.bg_fill, "Colors:Window", "BackgroundNormal");
    set_color!(style.visuals.widgets.noninteractive.weak_bg_fill, "Colors:Header", "BackgroundNormal");
    set_stroke!(style.visuals.widgets.noninteractive.fg_stroke, "Colors:Window", "ForegroundNormal", 1.);
    set_stroke!(style.visuals.widgets.noninteractive.fg_stroke, "WM", "activeForeground", 1.);
    // this is kinda a hack based on breeze, but it seems to work well enough on other themes
    set_stroke!(style.visuals.widgets.noninteractive.bg_stroke, "ColorEffects:Inactive", "Color", 1.);

    set_color!(style.visuals.widgets.inactive.bg_fill, "Colors:View", "BackgroundNormal"); // TODO dark
    set_color!(style.visuals.widgets.inactive.weak_bg_fill, "Colors:Button", "BackgroundNormal");
    set_stroke!(style.visuals.widgets.inactive.fg_stroke, "Colors:Button", "ForegroundNormal", 1.);
    set_stroke!(style.visuals.widgets.inactive.bg_stroke, "Colors:Button", "ForegroundInactive", 1.);

    set_color!(style.visuals.widgets.hovered.bg_fill, "Colors:Button", "BackgroundNormal");
    set_color!(style.visuals.widgets.hovered.weak_bg_fill, "Colors:Button", "BackgroundNormal");
    set_stroke!(style.visuals.widgets.hovered.fg_stroke, "Colors:Button", "ForegroundNormal", 1.);
    set_stroke!(style.visuals.widgets.hovered.bg_stroke, "Colors:Button", "DecorationHover", 1.);

    set_color!(style.visuals.widgets.active.bg_fill, "Colors:Button", "BackgroundAlternate");
    set_color!(style.visuals.widgets.active.weak_bg_fill, "Colors:Button", "BackgroundAlternate");
    set_stroke!(style.visuals.widgets.active.fg_stroke, "Colors:Button", "ForegroundNormal", 1.);
    set_stroke!(style.visuals.widgets.active.bg_stroke, "Colors:Button", "DecorationFocus", 1.);

    set_color!(style.visuals.widgets.open.bg_fill, "Colors:Button", "BackgroundAlternate");
    set_color!(style.visuals.widgets.open.weak_bg_fill, "Colors:Header", "BackgroundNormal");
    set_stroke!(style.visuals.widgets.open.fg_stroke, "Colors:Button", "ForegroundVisited", 1.);
    set_stroke!(style.visuals.widgets.open.bg_stroke, "Colors:Button", "DecorationFocus", 1.);


    set_color!(style.visuals.hyperlink_color, "Colors:Button", "ForegroundLink");
    set_color!(style.visuals.panel_fill, "Colors:Window", "BackgroundNormal");
    set_color!(style.visuals.panel_fill, "WM", "inactiveBackground");
    set_color!(style.visuals.window_fill, "Colors:Window", "BackgroundNormal");
    set_stroke!(style.visuals.window_stroke, "ColorEffects:Inactive", "Color", 1.);

    set_color!(style.visuals.code_bg_color, "Colors:View", "BackgroundNormal");
    set_color!(style.visuals.extreme_bg_color, "Colors:View", "BackgroundNormal");
    set_color!(style.visuals.faint_bg_color, "Colors:Tooltip", "BackgroundNormal"); // This is Header on breeze

    set_color!(style.visuals.selection.bg_fill, "Colors:Selection", "BackgroundAlternate");
    set_stroke!(style.visuals.selection.stroke, "Colors:Selection", "ForegroundNormal", 1.);

    // Some arbitrary changes i've hardcoded, since these things couldn't be gotten from kdeglobals
    // In my opinion it makes things look a little nicer when using breeze and the other color themes i have
    let shadow = Shadow {
        offset: vec2(0., 10.),
        blur: 30.,
        spread: 10.,
        color: Color32::from_rgba_premultiplied(0, 0, 0, 50),
    };
    style.visuals.popup_shadow = shadow;
    style.visuals.window_shadow = shadow;

    style.visuals.widgets.active.expansion = 0.;
    style.visuals.widgets.hovered.expansion = 0.;
    style.visuals.widgets.noninteractive.expansion = 0.;
    style.visuals.widgets.open.expansion = 0.;

    let rounding = Rounding::same(2.);
    style.visuals.menu_rounding = rounding;
    style.visuals.window_rounding = rounding;

    style.spacing.window_margin = Margin::same(2.);
    style.spacing.menu_margin = Margin::same(4.);
    style.spacing.button_padding = vec2(8., 3.);

    Ok(())
}

/// Modifies a style to use the current GTK(version) theme.
pub fn style_gtk(style: &mut Style, version: u8) -> Result<(), Box<dyn Error>> {
    // TODO fonts
    let mut gtk_settings = Ini::new();
    gtk_settings.load(
        Path::new(&env::var("HOME")?).join(format!(".config/gtk-{version}.0/settings.ini")),
    )?;
    let theme_name = gtk_settings
        .get("Settings", "gtk-theme-name")
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Could not get gtk-theme-name in gtk4 settings.ini",
            )
        })?;
    let dark_mode = *DARK_LIGHT_MODE != dark_light::Mode::Light;
    let css_file_name = if dark_mode { "gtk-dark.css" } else { "gtk.css" };
    let path = [
        // All paths the css file we're looking for could be, chooses the first one that exists
        Path::new(&env::var("HOME")?).join(format!(
            ".themes/{theme_name}/gtk-{version}.0/{css_file_name}"
        )),
        Path::new(&env::var("HOME")?).join(format!(".themes/{theme_name}/gtk-{version}.0/gtk.css")), // Fallback if we are in dark mode and gtk-dark.css does not exist
        format!("/usr/share/themes/{theme_name}/gtk-{version}.0/{css_file_name}").into(),
        format!("/usr/share/themes/{theme_name}/gtk-{version}.0/gtk.css").into(),
    ]
    .into_iter()
    .find(|path| path.exists())
    .ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Could not find gtk.css or gtk-dark.css file for theme {theme_name}"),
        )
    })?;

    gtk::style_gtk_css(style, &path, &mut gtk::GtkCssParseContext::default())?;

    Ok(())
}

pub trait IniExt {
    fn get_color(&self, section: &str, key: &str) -> Result<Color32, std::io::Error>;
}
impl IniExt for Ini {
    fn get_color(&self, section: &str, key: &str) -> Result<Color32, std::io::Error> {
        let input = self.get(section, key).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("color not found: section {section}, key: {key}"),
            )
        })?;

        let mut numbers = input.split(',').map(|n| n.trim().parse::<u8>());
        let mut get_number = || {
            numbers.next().map(Result::ok).flatten().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, format!("color {input}"))
            })
        };

        Ok(Color32::from_rgba_premultiplied(
            get_number()?,
            get_number()?,
            get_number()?,
            get_number().unwrap_or(255),
        ))
    }
}
