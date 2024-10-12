#![cfg(target_os = "macos")]

use crate::*;
use cocoa::{base::{id, nil}, foundation::NSString};
use objc::{msg_send, class, sel, sel_impl};

pub fn style(style: &mut Style) -> Result<(), Box<dyn Error>> {
    // TODO Currently only accent color is supported

    // text works better with the accent colors when it's more like the macos text color
    if dark_light::detect() == dark_light::Mode::Dark {
        style.visuals.override_text_color = Some(style.visuals.text_color().mutate(Rgba::WHITE, 0.7));
    }

    // if the accent color is Multicolor then it will not be set
    let highlight = match get_ns_accent_color() {
        Some(c) => c.into(),
        None => return Ok(()),
    };

    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1., highlight);
    style.visuals.widgets.active.bg_fill = highlight;
    style.visuals.widgets.active.weak_bg_fill = highlight;
    style.visuals.widgets.active.bg_stroke = Stroke::new(1., highlight);
    style.visuals.widgets.open.bg_fill = highlight;
    style.visuals.widgets.open.bg_stroke = Stroke::new(1., highlight);
    style.visuals.hyperlink_color = highlight;
    style.visuals.selection.bg_fill = highlight;

    Ok(())
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
