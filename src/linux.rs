use std::{collections::HashMap, env, error::Error, fs, io, path::Path};

use configparser::ini::Ini;
use epaint::Shadow;
use lightningcss::{printer::PrinterOptions, properties::{border::BorderSideWidth, custom::{Function, Token, TokenOrValue, UnparsedProperty}, Property, PropertyId}, rules::{style::StyleRule, unknown::UnknownAtRule, CssRule, CssRuleList}, selector::{Component, Selector}, stylesheet::{MinifyOptions, ParserOptions, StyleSheet}, targets::{Features, Targets}, traits::ToCss, values::{color::{CssColor, FloatColor, LABColor, PredefinedColor, HSL, HWB, RGBA, SRGB}, length::{Length, LengthValue}, percentage::DimensionPercentage}};
use palette::{IntoColor, WithAlpha};

use crate::*;

pub fn style(style: &mut Style) -> Result<(), Box<dyn Error>> {
    style_gtk(style, 4).or_else(|_| style_gtk(style, 3))?;
    return Ok(());
    
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
    set_stroke!(style.visuals.widgets.noninteractive.bg_stroke, "Colors:Window", "ForegroundNormal", 0.5); // TODO
    set_stroke!(style.visuals.widgets.noninteractive.bg_stroke, "WM", "inactiveBlend", 0.5);

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
    set_stroke!(style.visuals.window_stroke, "Colors:Window", "ForegroundNormal", 0.5);
    set_stroke!(style.visuals.window_stroke, "WM", "inactiveBlend", 0.5);
    style.visuals.window_shadow = Shadow {
        offset: vec2(0., 10.),
        blur: 30.,
        spread: 10.,
        color: Color32::from_rgba_premultiplied(0, 0, 0, 50),
    };

    set_color!(style.visuals.code_bg_color, "Colors:View", "BackgroundNormal");
    set_color!(style.visuals.extreme_bg_color, "Colors:View", "BackgroundNormal");
    set_color!(style.visuals.faint_bg_color, "Colors:Tooltip", "BackgroundNormal"); // This is Header on breeze

    set_color!(style.visuals.selection.bg_fill, "Colors:Selection", "BackgroundAlternate");
    set_stroke!(style.visuals.selection.stroke, "Colors:Selection", "ForegroundNormal", 1.);

    style.visuals.widgets.active.expansion = 0.;
    style.visuals.widgets.hovered.expansion = 0.;
    style.visuals.widgets.noninteractive.expansion = 0.;
    style.visuals.widgets.open.expansion = 0.;

    style.spacing.window_margin = Margin::same(2.);
    style.spacing.menu_margin = Margin::same(4.);

    Ok(())
}

// GTK themes are not nearly as easy to parse as KDE themes.
// We'll have to create part of a css interpreter to do this, luckily we have lightningcss to make us an AST!

/// Modifies a style to use the current GTK(version) theme.
pub fn style_gtk(style: &mut Style, version: u8) -> Result<(), Box<dyn Error>> {
    // TODO fonts
    let mut gtk_settings = Ini::new();
    gtk_settings.load(Path::new(&env::var("HOME")?).join(format!(".config/gtk-{version}.0/settings.ini")))?;
    let theme_name = gtk_settings.get("Settings", "gtk-theme-name").ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Could not get gtk-theme-name in gtk4 settings.ini"))?;
    let dark_mode = *DARK_LIGHT_MODE != dark_light::Mode::Light;
    let css_file_name = if dark_mode { "gtk-dark.css" } else { "gtk.css" };
    let path = [
        // All paths the css file we're looking for could be, chooses the first one that exists
        Path::new(&env::var("HOME")?).join(format!(".themes/{theme_name}/gtk-{version}.0/{css_file_name}")),
        Path::new(&env::var("HOME")?).join(format!(".themes/{theme_name}/gtk-{version}.0/gtk.css")), // Fallback if we are in dark mode and gtk-dark.css does not exist
        format!("/usr/share/themes/{theme_name}/gtk-{version}.0/{css_file_name}").into(),
        format!("/usr/share/themes/{theme_name}/gtk-{version}.0/gtk.css").into(),
    ].into_iter().find(|path| path.exists()).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("Could not find gtk.css or gtk-dark.css file for theme {theme_name}")))?;
    

    let mut ctx = GtkCssParseContext::default();
    style_gtk_css(style, &path, &mut ctx)?;
    // println!("{ctx:#?}");

    
    // let mut new_files = Vec::new();
    // resolve_css_imports(&mut stylesheet.rules, &path, &mut new_files)?;
    
    // println!("{:#?}", stylesheet);

    // style_gtk_css(style, &stylesheet.rules, &path)?;
    
    // let mut parser_input = ParserInput::new(&css_content);
    // let mut parser = Parser::new(&mut parser_input);
    // cssparser::
    // let rules = RuleBodyParser::new(&mut parser, &mut parser);

    Ok(())
}

macro_rules! css_values {
    {$($name:ident($ty:ty) $as_fn:ident),* $(,)?} => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum CssValue {
            $($name($ty),)*
        }

        $(impl From<$ty> for CssValue {
            fn from(value: $ty) -> Self {
                Self::$name(value)
            }
        })*

        pub trait CssValueConversions {
            $(fn $as_fn(self) -> Option<$ty>;)*
        }

        impl CssValueConversions for CssValue {
            $(fn $as_fn(self) -> Option<$ty> {
                match self {
                    Self::$name(value) => Some(value),
                    _ => None,
                }
            })*
        }

        impl CssValueConversions for Option<CssValue> {
            $(fn $as_fn(self) -> Option<$ty> {
                match self? {
                    CssValue::$name(value) => Some(value),
                    _ => None,
                }
            })*
        }
    };
}

css_values! {
    Color(Color32) as_color,
    Number(f32) as_number,
}

pub trait TokenOrValueIterExt {
    fn expect_token(&mut self, token: Token) -> Option<()>;
    fn eval(&mut self, ctx: &GtkCssParseContext) -> Option<CssValue>;
}
impl<'a, I: Iterator<Item = &'a TokenOrValue<'a>>> TokenOrValueIterExt for I {
    fn expect_token(&mut self, token: Token) -> Option<()> {
        let TokenOrValue::Token(next_token) = self.next()? else { return None };
        (next_token == &token).then_some(())
    }

    fn eval(&mut self, ctx: &GtkCssParseContext) -> Option<CssValue> {
        match self.next()? {
            TokenOrValue::Token(Token::AtKeyword(ident)) => ctx.defined_colors.get(ident.as_ref()).copied().map(CssValue::from),
            TokenOrValue::Token(Token::Ident(ident)) => COLOR_WORD_MAP.iter()
                .find(|(name, _)| ident.as_ref() == *name)
                .map(|(_, color)| CssValue::Color(*color)),
            TokenOrValue::Token(Token::Number { has_sign: _, value, .. }) => Some(CssValue::Number(*value)),
            TokenOrValue::Token(Token::Percentage { has_sign: _, unit_value, .. }) => Some(CssValue::Number(*unit_value)),
            TokenOrValue::Function(function) => ctx.eval_function(function),
            
            _ => None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GtkCssParseContext {
    pub defined_colors: HashMap<String, Color32>,
}
impl GtkCssParseContext {
    fn eval_function(&self, function: &Function) -> Option<CssValue> {
        let mut args = function.arguments.0.iter()
            .filter(|token| !matches!(token, TokenOrValue::Token(Token::WhiteSpace(_))));

        match function.name.as_ref() {
            "mix" => {
                let c1 = convert_to_srgb(args.eval(self).as_color()?);
                args.expect_token(Token::Comma)?;
                let c2 = convert_to_srgb(args.eval(self).as_color()?);
                args.expect_token(Token::Comma)?;
                let t = args.eval(self).as_number()?;

                Some(CssValue::Color(convert_srgb(SRGB {
                    r: lerp(c1.r..=c2.r, t),
                    g: lerp(c1.g..=c2.g, t),
                    b: lerp(c1.b..=c2.b, t),
                    alpha: lerp(c1.alpha..=c2.alpha, t),
                })))
            }
            // We need to turn a gradient into a single color, to do this we just choose the color closest to half way
            "linear-gradient" => {
                let mut values = Vec::new();
                
                args.next()?;
                while args.expect_token(Token::Comma).is_some() {
                    values.push(args.eval(self).as_color()?);
                    args.eval(self).as_number()?;
                }

                Some(CssValue::Color(*values.get(values.len() / 2)?))
            }
            // I found this function in the Breeze-Dark GTK3 theme, but i have no idea what it does,
            // I couldn't find anything online about it, so i'm just assuming it's setting the alpha
            "alpha" => {
                let color = args.eval(self).as_color()?;
                let alpha = (args.eval(self).as_number()? * 255.) as u8;

                Some(CssValue::Color(Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha)))
            }

            _ => None
        }
    }
    
    pub fn extract_background_color(&self, property: &Property) -> Option<Color32> {
        match property {
            Property::BackgroundColor(color) => Some(convert_css_color(color)),
            Property::Unparsed(UnparsedProperty { property_id: PropertyId::BackgroundColor | PropertyId::Background, value }) => {
                value.0.iter().eval(self).as_color()
            },
            Property::Background(background) => Some(convert_css_color(&background.first()?.color)),
            
            _ => None,
        }
    }

    pub fn extract_foreground_color(&self, property: &Property) -> Option<Color32> {
        match property {
            Property::Color(color) => Some(convert_css_color(color)),
            Property::Unparsed(UnparsedProperty { property_id: PropertyId::Color, value }) => {
                value.0.iter().eval(self).as_color()
            },
            
            _ => None,
        }
    }

    pub fn extract_border_color(&self, property: &Property) -> Option<Color32> {
        match property {
            Property::Border(border) => Some(convert_css_color(&border.color)),
            Property::BorderColor(color) => Some(convert_css_color(&color.top)),
            Property::Unparsed(UnparsedProperty { property_id: PropertyId::BorderColor | PropertyId::Border, value }) => {
                value.0.iter().eval(self).as_color()
            },
            
            _ => None,
        }
    }
    pub fn extract_border_width(&self, property: &Property) -> Option<f32> {
        match property {
            Property::Border(border) => Some(convert_border_side_width(&border.width)),
            Property::BorderWidth(width) => Some(convert_border_side_width(&width.top)),
            
            _ => None,
        }
    }
    pub fn extract_border_radius(&self, property: &Property) -> Option<Rounding> {
        match property {
            Property::BorderRadius(radius, _) => Some(Rounding {
                nw: convert_dimension_percentage(&radius.top_left.0),
                ne: convert_dimension_percentage(&radius.top_right.0),
                sw: convert_dimension_percentage(&radius.bottom_left.0),
                se: convert_dimension_percentage(&radius.bottom_right.0),
            }),
            
            _ => None,
        }
    }
    pub fn extract_border_into<const NUM: usize>(&self, property: &Property, dst: [(&mut Stroke, &mut Rounding); NUM]) {
        if let Some(border_color) = self.extract_border_color(property) {
            for (stroke, _) in dst {
                stroke.color = border_color;
            }
        } else if let Some(border_width) = self.extract_border_width(property) {
            for (stroke, _) in dst {
                stroke.width = border_width;
            }
        } else if let Some(border_radius) = self.extract_border_radius(property) {
            for (_, rounding) in dst {
                *rounding = border_radius;
            }
        }
    }
}

// O

#[rustfmt::skip]
pub fn style_gtk_css(style: &mut Style, path: &Path, ctx: &mut GtkCssParseContext) -> Result<(), Box<dyn Error>> {
    let css_content = fs::read_to_string(&path)?;

    let mut stylesheet = StyleSheet::parse(&css_content, ParserOptions {
        filename: path.display().to_string(),
        ..Default::default()
    }).map_err(|err| err.to_string())?;

    // Try to remove some complexity from all this
    stylesheet.minify(MinifyOptions {
        targets: Targets {
            include: Features::all(),
            // exclude: Features::all(),
            ..Default::default()
        },
        ..Default::default()
    })?;
    
    for rule in &stylesheet.rules.0 {
        match rule {
            CssRule::Import(rule) => {
                // Not sure if there's a better way to do this
                let dir_path = path.parent().expect("Invalid css path, this is a bug!");
                style_gtk_css(style, &dir_path.join(rule.url.to_string()), ctx)?;
            }
            CssRule::Style(rule) => style_gtk_rule(style, ctx, rule)?,
            CssRule::Unknown(rule) => {
                // Parse define-color rules
                if rule.name != "define-color" { continue }
                let mut prelude = rule.prelude.0.iter();
                let Some(TokenOrValue::Token(Token::Ident(ident))) = prelude.next() else { continue };
                let Some(TokenOrValue::Token(Token::WhiteSpace(_))) = prelude.next() else { continue };
                match prelude.next() {
                    Some(TokenOrValue::Color(CssColor::RGBA(rgba))) => {
                        ctx.defined_colors.insert(ident.to_string(), convert_rgba(*rgba));
                    }
                    Some(TokenOrValue::Token(Token::AtKeyword(keyword))) => {
                        let Some(color) = ctx.defined_colors.get(keyword.as_ref()) else { continue };
                        ctx.defined_colors.insert(ident.to_string(), color.clone());
                    }
                    
                    _ => {}
                }
            }
            rule => {
                // println!("Rule ignored: {rule:?}");
            }
        }
    }

    Ok(())
}

fn style_gtk_rule(style: &mut Style, ctx: &GtkCssParseContext, rule: &StyleRule) -> Result<(), Box<dyn Error>> {
    for selectors in &rule.selectors.0 {
        /* for selector in selectors.iter() {
            if let Component::Class(id) = selector {
                if id.to_string() == ".background" || id.to_string() == "background" || id.to_string() == "headerbar" || id.to_string() == "button" {
                    println!("{id:?}");
                }
            }
        } */
        // let selectors = selectors.iter().collect::<Vec<_>>();
        // TODO we should probably do this in a better way then just checking the string and a bunch of if elses
        let selector = selectors.to_css_string(printer_options())?;

        if selector == ".background" {
            for (property, _important) in rule.declarations.iter() {
                if let Some(bg_color) = ctx.extract_background_color(property) {
                    style.visuals.panel_fill = bg_color;
                    style.visuals.window_fill = bg_color;
                }
                if let Some(fg_color) = ctx.extract_foreground_color(property) {
                    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1., fg_color);
                }
            }
        } else if selector == "headerbar" {
            for (property, _important) in rule.declarations.iter() {
                if let Some(bg_color) = ctx.extract_background_color(property) {
                    style.visuals.widgets.open.weak_bg_fill = bg_color;
                    style.visuals.widgets.open.bg_fill = bg_color;
                }
                if let Some(fg_color) = ctx.extract_foreground_color(property) {
                    style.visuals.widgets.open.fg_stroke = Stroke::new(1., fg_color);
                }
            }
        } else if selector == "button" {
            for (property, _important) in rule.declarations.iter() {
                if let Some(bg_color) = ctx.extract_background_color(property) {
                    style.visuals.widgets.inactive.weak_bg_fill = bg_color;
                    style.visuals.widgets.inactive.bg_fill = bg_color;
                    style.visuals.widgets.hovered.weak_bg_fill = bg_color;
                    style.visuals.widgets.hovered.bg_fill = bg_color;
                    style.visuals.widgets.active.weak_bg_fill = bg_color;
                    style.visuals.widgets.active.bg_fill = bg_color;
                }
                if let Some(fg_color) = ctx.extract_foreground_color(property) {
                    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1., fg_color);
                    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1., fg_color);
                    style.visuals.widgets.active.fg_stroke = Stroke::new(1., fg_color);
                }
                ctx.extract_border_into(property, [
                    (&mut style.visuals.widgets.inactive.bg_stroke, &mut style.visuals.widgets.inactive.rounding),
                    (&mut style.visuals.widgets.hovered.bg_stroke, &mut style.visuals.widgets.hovered.rounding),
                    (&mut style.visuals.widgets.active.bg_stroke, &mut style.visuals.widgets.active.rounding),
                ]);
            }
        } else if selector == "button:hover" {
            // I'm assuming that button comes before button:hover here
            println!("{rule:#?}");
            for (property, _important) in rule.declarations.iter() {
                if let Some(bg_color) = ctx.extract_background_color(property) {
                    style.visuals.widgets.hovered.weak_bg_fill = bg_color;
                    style.visuals.widgets.hovered.bg_fill = bg_color;
                }
                if let Some(fg_color) = ctx.extract_foreground_color(property) {
                    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1., fg_color);
                }
                ctx.extract_border_into(property, [(&mut style.visuals.widgets.hovered.bg_stroke, &mut style.visuals.widgets.hovered.rounding)]);
            }
        } else if selector == "button:active" {
            println!("{rule:#?}");
            for (property, _important) in rule.declarations.iter() {
                if let Some(bg_color) = ctx.extract_background_color(property) {
                    style.visuals.widgets.active.weak_bg_fill = bg_color;
                    style.visuals.widgets.active.bg_fill = bg_color;
                }
                if let Some(fg_color) = ctx.extract_foreground_color(property) {
                    style.visuals.widgets.active.fg_stroke = Stroke::new(1., fg_color);
                }
                ctx.extract_border_into(property, [(&mut style.visuals.widgets.active.bg_stroke, &mut style.visuals.widgets.active.rounding)]);
            }
        }
    }

    Ok(())
}

pub const fn convert_rgba(rgba: RGBA) -> Color32 {
    Color32::from_rgba_premultiplied(rgba.red, rgba.green, rgba.blue, rgba.alpha)
}
pub fn convert_srgb(srgb: SRGB) -> Color32 {
    Color32::from_rgba_premultiplied((srgb.r * 255.) as u8, (srgb.g * 255.) as u8, (srgb.b * 255.) as u8, (srgb.alpha * 255.) as u8)
}
pub fn convert_to_srgb(color: Color32) -> SRGB {
    SRGB { r: color.r() as f32 / 255., g: color.g() as f32 / 255., b: color.b() as f32 / 255., alpha: color.a() as f32 / 255. }
}
fn palette_convert(color: palette::Alpha<palette::Srgb, f32>) -> Color32 {
    convert_srgb(SRGB { r: color.red, g: color.green, b: color.blue, alpha: color.alpha })
}
pub fn convert_border_side_width(width: &BorderSideWidth) -> f32 {
    match width {
        BorderSideWidth::Thin => 0.5,
        BorderSideWidth::Medium => 1.,
        BorderSideWidth::Thick => 2.,
        BorderSideWidth::Length(len) => convert_length(len),
    }
}
pub fn convert_length(len: &Length) -> f32 {
    match len {
        Length::Value(value) => value.to_px().unwrap_or(1.),
        Length::Calc(_) => 1., // TODO i do not want to deal with interpreting this right now
    }
}
pub fn convert_dimension_percentage(value: &DimensionPercentage<LengthValue>) -> f32 {
    match value {
        DimensionPercentage::Dimension(value) => value.to_px().unwrap_or(1.),
        DimensionPercentage::Percentage(p) => p.0,
        DimensionPercentage::Calc(_) => 1.,
    }
}

#[rustfmt::skip]
pub fn convert_css_color(color: &CssColor) -> Color32 {
    match color {
        CssColor::CurrentColor => Color32::DARK_GRAY, // TODO
        CssColor::RGBA(rgba) => convert_rgba(*rgba),
        CssColor::LAB(lab) => match lab.as_ref().clone() {
            LABColor::LAB(c) => convert_rgba(c.into()),
            LABColor::LCH(c) => convert_rgba(c.into()),
            LABColor::OKLAB(c) => convert_rgba(c.into()),
            LABColor::OKLCH(c) => convert_rgba(c.into()),
        }
        CssColor::Predefined(c) => match c.as_ref() {
            PredefinedColor::SRGB(srgb) => convert_srgb(*srgb),
            PredefinedColor::SRGBLinear(c) => palette_convert(palette::LinSrgba::new(c.r, c.g, c.b, c.alpha).into_color()),
            _ => Color32::DARK_GRAY,
        },
        CssColor::Float(c) => match c.as_ref() {
            FloatColor::RGB(srgb) => convert_srgb(*srgb),
            FloatColor::HSL(hsl) => palette_convert(palette::Hsl::new(hsl.h, hsl.s, hsl.l).with_alpha(hsl.alpha).into_color()),
            FloatColor::HWB(hwb) => palette_convert(palette::Hwb::new(hwb.h, hwb.w, hwb.b).with_alpha(hwb.alpha).into_color()),
        },
        CssColor::LightDark(light, dark) => if *DARK_LIGHT_MODE == dark_light::Mode::Light { convert_css_color(&light) } else { convert_css_color(&dark) },
        CssColor::System(_) => Color32::DARK_GRAY, // TODO
    }
}

fn printer_options<'a>() -> PrinterOptions<'a> {
    PrinterOptions {
        minify: true,
        ..Default::default()
    }
}

/* fn resolve_css_imports<'a>(rules: &mut CssRuleList<'a>, path: &Path, new_files: &'a mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut replaced = Vec::new();
    
    for (i, rule) in rules.0.iter().enumerate() {
        if let CssRule::Import(rule) = rule {
            replaced.push((i, rule.url.clone()))
        }
    }

    for (i, new_path) in replaced.into_iter().rev() {
        let path = path.join(new_path.to_string());
        new_files.push(fs::read_to_string(&path)?);
        
        let mut stylesheet = StyleSheet::parse(new_files.last().unwrap(), ParserOptions {
            filename: path.display().to_string(),
            ..Default::default()
        }).map_err(|err| err.to_string())?;
    
        resolve_css_imports(&mut stylesheet.rules, &path, new_files)?;
        
        rules.0.splice(i..=i, stylesheet.rules.0.into_iter());
    }

    Ok(())
} */
