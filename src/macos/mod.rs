use crate::*;
use cocoa::{base::{id, nil}, foundation::NSString};
use objc::{msg_send, class, sel, sel_impl};
#[cfg(all(feature = "dynamic-mac-colors", target_os = "macos"))]
mod dynamic;

pub fn style(style: &mut Style) -> Result<(), Box<dyn Error>> {
    // todo make buttons and others bigger

    // use dynamic color if available
    #[cfg(all(feature = "dynamic-mac-colors", target_os = "macos"))]
    {
        set_accent(style);
        return dynamic::style(style);
    }

    #[cfg(not(feature = "dynamic-mac-colors"))]
    {
        static_style(style)
    }
}

pub fn static_style(style: &mut Style) -> Result<(), Box<dyn Error>> {
    style.visuals.widgets.hovered.expansion = 0.0;

    // text works better with the accent colors when it's more like the macos text color
    if *DARK_LIGHT_MODE == dark_light::Mode::Dark {
        style.visuals.override_text_color = Some(style.visuals.text_color().mutate(Rgba::WHITE, 0.7));

        let fill = Color32::from_rgb(42, 42, 42); // background color of dark mode appkit apps
        style.visuals.panel_fill = fill;
        style.visuals.window_fill = fill;
        // widget_text is also used for window borders according to the documentation
        style.visuals.window_stroke = Stroke::new(1., fill);

        style.visuals.widgets.noninteractive.bg_fill = fill.mutate(Rgba::BLACK, 0.1);
        style.visuals.widgets.noninteractive.weak_bg_fill = fill; // Used for text input hints and selected windows
        style.visuals.widgets.inactive.bg_fill = fill;
        style.visuals.widgets.inactive.weak_bg_fill = fill;
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1., Color32::GRAY.mutate(Rgba::WHITE, 0.1));
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(1., Color32::from_rgb(87, 87, 87));
        style.visuals.extreme_bg_color = Color32::from_rgb(54, 54, 54);
    }

    set_accent(style);

    Ok(())
}

fn set_accent(style: &mut Style) {
    // if the accent color is Multicolor then it will be set to blue as MacOS does
    let highlight: Color32 = get_ns_accent_color().unwrap_or(AccentColor::Blue).into();
    // improve legibliety
    let highlight_fill = highlight.mutate(Rgba::BLACK, 0.1);

    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1., highlight);
    style.visuals.widgets.active.bg_fill = highlight;
    style.visuals.widgets.active.weak_bg_fill = highlight_fill;
    style.visuals.widgets.active.bg_stroke = Stroke::new(1., highlight);
    style.visuals.widgets.open.bg_fill = highlight_fill;
    style.visuals.widgets.open.bg_stroke = Stroke::new(1., highlight);
    style.visuals.hyperlink_color = highlight_fill;
    style.visuals.selection.bg_fill = highlight_fill;
}

/// Takes the `Color32` consts and converts them to the MacOS specified versions.\
/// The color will be unchanged if it's not one of the consts.\
/// https://developer.apple.com/design/human-interface-guidelines/color#Specifications
pub fn color32_to_macos_color(color: Color32) -> Color32 {
    if *DARK_LIGHT_MODE == dark_light::Mode::Dark {
        match color {
            Color32::BLUE => Color32::from_rgb(10, 132, 255),
            Color32::BROWN => Color32::from_rgb(172, 142, 104),
            Color32::GRAY => Color32::from_rgb(152, 152, 157),
            Color32::GREEN => Color32::from_rgb(50, 215, 75),
            Color32::ORANGE => Color32::from_rgb(255, 159, 10),
            Color32::RED => Color32::from_rgb(255, 69, 58),
            Color32::YELLOW => Color32::from_rgb(255, 214, 10),
            _ => match_dark_light_colors(color)
        }
    } else {
        match color {
            Color32::BLUE => Color32::from_rgb(0, 122, 255),
            Color32::BROWN => Color32::from_rgb(162, 132, 94),
            Color32::GRAY => Color32::from_rgb(142, 142, 147),
            Color32::GREEN => Color32::from_rgb(40, 205, 65),
            Color32::ORANGE => Color32::from_rgb(255, 149, 0),
            Color32::RED => Color32::from_rgb(255, 59, 48),
            Color32::YELLOW => Color32::from_rgb(255, 204, 0),
            _ => match_dark_light_colors(color)
        }
    }
}

/// dark colors are the light mode accessible colors, light colors are the inverse.\
/// The color will be unchanged if it's not one of the consts.
const fn match_dark_light_colors(color: Color32) -> Color32 {
    match color {
        Color32::DARK_BLUE => Color32::from_rgb(0, 64, 221),
        Color32::DARK_GRAY => Color32::from_rgb(105, 105, 110),
        Color32::DARK_GREEN => Color32::from_rgb(0, 125, 27),
        Color32::DARK_RED => Color32::from_rgb(215, 0, 21),
        Color32::LIGHT_BLUE => Color32::from_rgb(90, 200, 245), // Cyan
        Color32::LIGHT_GRAY => Color32::from_rgb(152, 152, 157),
        Color32::LIGHT_RED => Color32::from_rgb(255, 105, 97),
        Color32::LIGHT_YELLOW => Color32::from_rgb(255, 212, 38),
        _ => color
    }
}

// the AccentColor code is from [dan-lee/tao](https://github.com/dan-lee/tao/tree/feat/accent_color_macos)

/// The different macos accent colors
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AccentColor {
    Graphite,
    Red,
    Orange,
    // ilegible
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

/// Convert an `AccentColor` to a `Color32` use numbers from the settings samples in macos
impl From<AccentColor> for Color32 {
    fn from(color: AccentColor) -> Self {
        // colors from the settings samples
        match color {
            AccentColor::Graphite => Color32::from_rgb(140, 140, 140),
            AccentColor::Red => Color32::from_rgb(236, 95, 93),
            AccentColor::Orange => Color32::from_rgb(232, 136, 58),
            AccentColor::Yellow => Color32::from_rgb(246, 200, 68),
            AccentColor::Green => Color32::from_rgb(120, 184, 86),
            AccentColor::Blue => Color32::from_rgb(52, 120, 246),
            AccentColor::Purple => Color32::from_rgb(155, 85, 163),
            AccentColor::Pink => Color32::from_rgb(228, 92, 156),
        }
    }
}

/// Get the system accent color using ojbc
fn get_ns_accent_color() -> Option<AccentColor> {
    let color_int: id;
    unsafe {
        let key_name = NSString::alloc(nil).init_str("AppleAccentColor");

        let user_defaults: id = msg_send![class!(NSUserDefaults), standardUserDefaults];
        let color_obj: id = msg_send![user_defaults, objectForKey: key_name];

        if color_obj == nil {
            return None;
        }

        color_int = msg_send![user_defaults, integerForKey: key_name];
    }

    match color_int as i8 {
        -1 => Some(AccentColor::Graphite),
        0 => Some(AccentColor::Red),
        1 => Some(AccentColor::Orange),
        2 => Some(AccentColor::Yellow),
        3 => Some(AccentColor::Green),
        4 => Some(AccentColor::Blue),
        5 => Some(AccentColor::Purple),
        6 => Some(AccentColor::Pink),
        _ => None,
    }
}
