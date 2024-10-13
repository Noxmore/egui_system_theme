#![cfg(all(feature = "dynamic-mac-colors", target_os = "macos"))]

use crate::*;
pub(crate) use ffi::{colors, Colors};

#[swift_bridge::bridge]
mod ffi {
    // The difarent things you can request the color of
    pub(crate) enum Colors {
        Text,
        Link,
        Black,
        Red,
        White,
        Clear,
        Blue,
        Gray,
        Green,
        Primary,
        Accent,
        Secondary,
        Yellow,
        Brown,
        Cyan,
        Indigo,
        Mint,
        Orange,
        Pink,
        Purple,
        Teal,
        Separator,
        TextEdit,
        Shadow,
        InputCursor,
        Window,
        InactiveFg,
        Stripe,
    }

    extern "Swift" {
        pub fn colors(color: Colors) -> (f64, f64, f64, f64);
    }
}

fn swift_to_color32(colors: (f64, f64, f64, f64)) -> Color32 {
    // maitily the 0-1 to make it 0-255
    // 0.? * 255.999 as u8
    Color32::from_rgb(
        (colors.0 * 255.999) as u8,
        (colors.1 * 255.999) as u8,
        (colors.2 * 255.999) as u8,
    )
}

/// get the ui color from swift
macro_rules! get_color {
    ($item:expr) => {
        swift_to_color32(colors($item))
    };
}

pub(crate) fn style(style: &mut Style) -> Result<(), Box<dyn Error>> {
    style.visuals.override_text_color = Some(get_color!(Colors::Text));
    style.visuals.hyperlink_color = get_color!(Colors::Link);

    style.visuals.widgets.hovered.expansion = 0.0;
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1., get_color!(Colors::InactiveFg));
    style.visuals.extreme_bg_color = get_color!(Colors::TextEdit);
    style.visuals.faint_bg_color = get_color!(Colors::Stripe).mutate(Rgba::WHITE, 0.05);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1., Color32::WHITE.mutate(get_color!(Colors::Accent).into(), 0.1));

    if *DARK_LIGHT_MODE == dark_light::Mode::Dark {
        // check box
        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(95, 95, 95);
        style.visuals.widgets.active.bg_fill =  Color32::from_rgb(95, 95, 95).mutate(get_color!(Colors::Accent).into(), 0.2);
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(0.34, Color32::from_rgb(120, 120, 120));
    } else {
        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(214, 214, 214);
        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(214, 214, 214).mutate(get_color!(Colors::Accent).into(), 0.2);
    }

    style.visuals.text_cursor.stroke.color = get_color!(Colors::InputCursor);

    let fill = get_color!(Colors::Window);
    style.visuals.panel_fill = fill;
    style.visuals.window_fill = fill;
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(0.45, get_color!(Colors::Separator));

    let highlight: Color32 = get_color!(Colors::Accent);
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
    style.visuals.widgets.hovered.bg_fill = highlight_fill;

    Ok(())
}

pub(crate) fn color32_to_macos_color(color: Color32) -> Color32 {
    match color {
        Color32::BLUE => get_color!(Colors::Blue),
        Color32::BROWN => get_color!(Colors::Brown),
        Color32::GRAY => get_color!(Colors::Gray),
        Color32::GREEN => get_color!(Colors::Green),
        Color32::ORANGE => get_color!(Colors::Orange),
        Color32::RED => get_color!(Colors::Red),
        Color32::YELLOW => get_color!(Colors::Yellow),
        Color32::TRANSPARENT => get_color!(Colors::Clear),
        _ => macos::match_dark_light_colors(color),
    }
}
