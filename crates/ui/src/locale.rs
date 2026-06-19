use gpui::Global;
use fluent_templates::{Loader, static_loader};
use unic_langid::{langid, LanguageIdentifier};

static_loader! {
    pub static LOCALES = {
        locales: "../../assets/locales",
        fallback_language: "en",
    };
}

/// A global localization manager responsible for translating UI strings and managing the application language state.
#[derive(Clone)]
pub struct Locale {
    pub language: LanguageIdentifier,
}

impl Global for Locale {}

impl Locale {
    /// Constructs a new `Locale` manager initialized with English ("en") as the default language.
    ///
    /// # Returns
    ///
    /// * An initialized `Locale` state instance.
    pub fn new() -> Self {
        Self {
            language: langid!("en"),
        }
    }

    /// Looks up a localization translation resource string matching the provided key identifier.
    ///
    /// If the key is missing in the primary and fallback languages, it prints a warning to the console and safely returns the raw key string to avoid UI crashes.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice referencing the unique translation key.
    ///
    /// # Returns
    ///
    /// * A resolved heap-allocated `String` translation text.
    pub fn translate(&self, key: &str) -> String {
        LOCALES.try_lookup(&self.language, key)
            .unwrap_or_else(|| {
                println!("[Localization Warning] Key '{}' not found!", key);
                key.to_string() 
            })
    }

    /// Toggles the active application language between Russian and English layouts.
    ///
    /// # Arguments
    ///
    /// * `cx` - A mutable reference to the global GPUI `App` application instance context.
    pub fn toggle_language(&self, cx: &mut gpui::App) {
        let locale = cx.global::<Self>().clone();
        
        let new_language = if locale.language == langid!("ru") {
            langid!("en")
        } else {
            langid!("ru")
        };

        cx.set_global(Locale { language: new_language });
        cx.refresh_windows();
    }
}

/// A short global utility helper function used to fetch localized strings swiftly.
///
/// # Arguments
///
/// * `cx` - A shared reference to the GPUI application lifecycle state context.
/// * `key` - The unique translation dictionary lookup identifier key token.
///
/// # Returns
///
/// * A thread-safe, immutable `SharedString` compatible with GPUI primitive rendering components.
pub fn tr(cx: &gpui::App, key: &str) -> gpui::SharedString {
    cx.global::<Locale>().translate(key).into()
}