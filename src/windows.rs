use crate::*;
use ::windows;
use epaint::Shadow;
use windows::Win32::Graphics::Gdi::*;

/// Gets a UI color from the windows API using [GetSysColor](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
pub(crate) fn get_color(index: SYS_COLOR_INDEX) -> Color32 {
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolorbrush
    // if unsafe { GetSysColorBrush(index).is_invalid() } {
    //     return None;
    // }
    unpack_0bgr(unsafe { GetSysColor(index) })
}

/// https://learn.microsoft.com/en-us/windows/win32/gdi/colorref
pub(crate) fn unpack_0bgr(packed: u32) -> Color32 {
    Color32::from_rgb(
        ((packed) & 0xff) as u8,
        ((packed >> 8) & 0xff) as u8,
        ((packed >> 16) & 0xff) as u8,
    )
}

const WINDOWS_ELEVEN_BUILD_NUMBER: u32 = 22000;

pub fn is_windows_eleven() -> bool {
    WINDOWS_ELEVEN_BUILD_NUMBER
        <= sysinfo::System::kernel_version()
            .unwrap_or_default()
            .parse()
            .unwrap_or(0)
}

pub fn style(style: &mut Style) -> Result<(), Box<dyn Error>> {
    // See https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#windows-1011-system-colors for color reference
    let window_background = get_color(COLOR_WINDOW);
    let window_text = get_color(COLOR_WINDOWTEXT);
    // let hyperlink = get_color(COLOR_HOTLIGHT);
    let inactive = get_color(COLOR_GRAYTEXT);
    // TODO this blends into the background a lot, i tried adding .mutate(Rgba::BLACK, 0.2) to it which helps in some cases, and breaks in others
    let highlight_text = get_color(COLOR_HIGHLIGHTTEXT);
    let highlight = get_color(COLOR_HIGHLIGHT);
    let widget_text = get_color(COLOR_BTNTEXT);
    let widget_background = get_color(COLOR_3DFACE);
    let widget_background_darker = widget_background.mutate(Rgba::BLACK, 0.1);

    // Modern windows is more flatly colored, this should help with that
    let window_divider = Stroke::new(1., window_background.mutate(Rgba::BLACK, 0.2));

    style.visuals.widgets.noninteractive.bg_fill = window_divider.color; // Not sure what this is used for
    style.visuals.widgets.noninteractive.weak_bg_fill = window_background; // Used for text input hints and selected windows
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1., window_text);
    style.visuals.widgets.noninteractive.bg_stroke = window_divider;

    style.visuals.widgets.inactive.bg_fill = window_divider.color;
    style.visuals.widgets.inactive.weak_bg_fill = widget_background;
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1., widget_text);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1., inactive);

    let hovered_background = widget_background.mutate(highlight.into(), 0.4);
    style.visuals.widgets.hovered.bg_fill = hovered_background;
    style.visuals.widgets.hovered.weak_bg_fill = hovered_background;
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1., widget_text);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1., highlight);

    style.visuals.widgets.active.bg_fill = highlight;
    style.visuals.widgets.active.weak_bg_fill = highlight;
    style.visuals.widgets.active.fg_stroke = Stroke::new(1., highlight_text);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1., highlight);

    style.visuals.widgets.open.bg_fill = highlight;
    style.visuals.widgets.open.weak_bg_fill = widget_background;
    style.visuals.widgets.open.fg_stroke = Stroke::new(1., highlight_text);
    style.visuals.widgets.open.bg_stroke = Stroke::new(1., highlight);

    style.visuals.hyperlink_color = highlight;
    style.visuals.panel_fill = widget_background_darker;
    style.visuals.window_fill = widget_background_darker;
    // widget_text is also used for window borders according to the documentation
    style.visuals.window_stroke = Stroke::new(1., widget_text);

    // let darker = window_background.mutate(Rgba::BLACK, 0.3);
    style.visuals.code_bg_color = window_background;
    style.visuals.extreme_bg_color = window_background;
    style.visuals.faint_bg_color = window_background.mutate(Rgba::from_gray(0.5), 0.2);

    style.visuals.selection.stroke = Stroke::new(1., highlight_text);
    style.visuals.selection.bg_fill = highlight;

    // Windows 10 doesn't have rounding on widgets
    if !is_windows_eleven() {
        style.visuals.widgets.noninteractive.rounding = Rounding::ZERO;
        style.visuals.widgets.inactive.rounding = Rounding::ZERO;
        style.visuals.widgets.hovered.rounding = Rounding::ZERO;
        style.visuals.widgets.active.rounding = Rounding::ZERO;
        style.visuals.widgets.open.rounding = Rounding::ZERO;
        style.visuals.window_rounding = Rounding::ZERO;
        style.visuals.menu_rounding = Rounding::ZERO;
    }

    // TODO linux popup shadow and menu rounding and button padding

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

    style.spacing.window_margin = Margin::same(2.);
    // Modern windows is generally more spaced out then egui's defaults
    style.spacing.button_padding = vec2(10., 3.);
    style.spacing.item_spacing = vec2(10., 6.);

    Ok(())
}
