use std::{ops::Deref, sync::Arc};

use crate::{App, Global, Rgba, Window, WindowAppearance, rgb};

/// The default set of colors for gpui.
///
/// These are used for styling base components, examples and more.
#[derive(Clone, Debug)]
pub struct Colors {
    /// Text color
    pub text: Rgba,
    /// Selected text color
    pub selected_text: Rgba,
    /// Background color
    pub background: Rgba,
    /// Disabled color
    pub disabled: Rgba,
    /// Selected color
    pub selected: Rgba,
    /// Border color
    pub border: Rgba,
    /// Separator color
    pub separator: Rgba,
    /// Container color
    pub container: Rgba,
}

impl Default for Colors {
    fn default() -> Self {
        Self::light()
    }
}

impl Colors {
    /// Returns the default colors for the given window appearance.
    pub fn for_appearance(window: &Window) -> Self {
        match window.appearance() {
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::light(),
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::dark(),
        }
    }

    /// Returns the default dark colors.
    pub fn dark() -> Self {
        Self {
            text: rgb(0xFFFFFF),
            selected_text: rgb(0xFFFFFF),
            disabled: rgb(0x565656),
            selected: rgb(0x2457CA),
            background: rgb(0x222222),
            border: rgb(0x000000),
            separator: rgb(0xD9D9D9),
            container: rgb(0x262626),
        }
    }

    /// Returns the default light colors.
    pub fn light() -> Self {
        Self {
            text: rgb(0x252525),
            selected_text: rgb(0xFFFFFF),
            background: rgb(0xFFFFFF),
            disabled: rgb(0xB0B0B0),
            selected: rgb(0x2A63D9),
            border: rgb(0xD9D9D9),
            separator: rgb(0xE6E6E6),
            container: rgb(0xF4F5F5),
        }
    }

    /// Get [Colors] from the global state
    pub fn get_global(cx: &App) -> &Arc<Colors> {
        &cx.global::<GlobalColors>().0
    }
}

/// Get [Colors] from the global state
#[derive(Clone, Debug)]
pub struct GlobalColors(pub Arc<Colors>);

impl Deref for GlobalColors {
    type Target = Arc<Colors>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Global for GlobalColors {}

/// Implement this trait to allow global [Colors] access via
/// `cx.default_colors()`.
pub trait DefaultColors {
    /// Returns the default [`Colors`]
    fn default_colors(&self) -> &Arc<Colors>;
}

impl DefaultColors for App {
    fn default_colors(&self) -> &Arc<Colors> {
        &self.global::<GlobalColors>().0
    }
}

/// The appearance of the base GPUI colors, used to style GPUI elements
///
/// Varies based on the system's current [`WindowAppearance`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DefaultAppearance {
    /// Use the set of colors for light appearances.
    #[default]
    Light,
    /// Use the set of colors for dark appearances.
    Dark,
}

impl From<WindowAppearance> for DefaultAppearance {
    fn from(appearance: WindowAppearance) -> Self {
        match appearance {
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
        }
    }
}
