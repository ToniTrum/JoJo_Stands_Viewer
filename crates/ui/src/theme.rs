use gpui::{App, Global, Hsla, hsla};

/// Represents the active visual mode of the application's user interface.
#[derive(Clone)]
enum ThemeMode {
    Light,
    Dark
}

/// A comprehensive styling configuration containing color palettes and opacity settings for the UI.
#[derive(Clone)]
pub struct Theme {
    mode: ThemeMode,

    pub background_color: u32,
    pub text_color: u32,

    pub button_color: u32,
    pub button_hover_color: u32,

    pub grid_color: u32,
    pub polygon_color: u32,
    pub polygon_opacity: u8,
    pub radar_text_color: Hsla,
}

impl Global for Theme {}

impl Theme {
    /// Constructs a predefined dark theme configuration matching low-light environment palettes.
    ///
    /// # Returns
    ///
    /// * A `Theme` instance initialized with dark background colors and high-contrast text markers.
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,

            background_color: 0x1e1e1e,
            text_color: 0xffffff,

            button_color: 0x2d2d2d,
            button_hover_color: 0x37373d,

            grid_color: 0x9d9d9d,
            polygon_color: 0xff830f,
            polygon_opacity: 120,
            radar_text_color: hsla(0.0, 1.0, 0.99, 1.0),
        }
    }

    /// Constructs a predefined light theme configuration matching high-luminance environment palettes.
    ///
    /// # Returns
    ///
    /// * A `Theme` instance initialized with light background colors and readable dark accents.
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,

            background_color: 0xffffff,
            text_color: 0x1e1e1e,

            button_color: 0xdadada,
            button_hover_color: 0xb7b7b7,

            grid_color: 0xafa9a9,
            polygon_color: 0xff830f,
            polygon_opacity: 120,
            radar_text_color: hsla(0.0, 0.0, 0.0, 1.0),
        }
    }

    /// Toggles the global application theme between Light and Dark modes.
    ///
    /// # Arguments
    ///
    /// * `cx` - A mutable reference to the GPUI `App` context.
    pub fn toggle_theme(&self, cx: &mut App) {
        let current = cx.global::<Self>().clone();

        let new_theme = match current.mode {
            ThemeMode::Light => Self::dark(),
            ThemeMode::Dark => Self::light(),
        };

        cx.set_global(new_theme);
        cx.refresh_windows();
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}