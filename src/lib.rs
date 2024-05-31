use std::error::Error;

pub use dark_light;
pub(crate) use egui::*;
pub(crate) use once_cell::sync::Lazy;

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

/// A shortcut to create a top panel with the id specified that mimics the system titlebar to avoid boilerplate
pub fn menu_bar<R>(
    ctx: &Context,
    id: impl Into<Id>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    #[cfg(target_os = "windows")]
    let fill_color = ctx.style().visuals.panel_fill;
    #[cfg(not(target_os = "windows"))]
    let fill_color = if ctx.input(|i| i.focused) {
        ctx.style().visuals.widgets.noninteractive.weak_bg_fill
    } else {
        ctx.style().visuals.panel_fill
    };

    TopBottomPanel::top(id)
        .frame(
            Frame::side_top_panel(&ctx.style())
                .fill(fill_color)
                .inner_margin(Margin::same(0.)),
        )
        .show(ctx, |ui| {
            let title_bar_response =
                ui.interact(ui.max_rect(), Id::new("menu_bar"), Sense::click_and_drag());

            if title_bar_response.double_clicked() {
                let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
                ctx.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
            }

            if title_bar_response.is_pointer_button_down_on()
                && title_bar_response.drag_motion() != Vec2::ZERO
            {
                ctx.send_viewport_cmd(ViewportCommand::StartDrag);
            }

            menu::bar(ui, |ui| {
                let style = ui.style_mut();
                #[cfg(target_os = "linux")]
                {
                    style.spacing.button_padding = vec2(10.0, 6.0);
                }
                #[cfg(not(target_os = "linux"))]
                {
                    style.spacing.button_padding = vec2(7.0, 4.0);
                }
                // For some themes, the button background is the same as the header background
                style.visuals.widgets.hovered.weak_bg_fill = style
                    .visuals
                    .widgets
                    .hovered
                    .weak_bg_fill
                    .mutate(Rgba::from_gray(0.5), 0.05);

                add_contents(ui)
            })
            .inner
        })
}

/* pub trait VisualsExt {
    /// The color of the window titlebar when using system theme.
    ///
    /// If the current platform is windows, `focused` will not be used, as a system call will be used instead of a visuals color.
    fn titlebar(&self, focused: bool) -> Color32;
}
impl VisualsExt for Visuals {
    #[allow(unused)]
    fn titlebar(&self, focused: bool) -> Color32 {
        // #[cfg(target_os = "windows")] {
        //     let mut packed = 0u32;
        //     if unsafe { ::windows::Win32::Graphics::Dwm::DwmGetColorizationColor(&mut packed, &mut Default::default()) }.is_ok() {
        //         return windows::unpack_argb(packed);
        //     }
        // }

        if focused {
            self.widgets.noninteractive.weak_bg_fill
        } else {
            self.panel_fill
        }
    }
} */

pub(crate) trait Color32Ext {
    fn mutate(self, towards: Rgba, amount: f32) -> Self;
}
impl Color32Ext for Color32 {
    fn mutate(self, towards: Rgba, amount: f32) -> Self {
        lerp(Rgba::from(self)..=towards, amount).into()
    }
}
