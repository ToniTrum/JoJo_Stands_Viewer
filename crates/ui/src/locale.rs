use gpui::Global;
use fluent_templates::{Loader, static_loader};
use unic_langid::{langid, LanguageIdentifier};

static_loader! {
    pub static LOCALES = {
        locales: "../../assets/locales",
        fallback_language: "en",
    };
}

#[derive(Clone)]
pub struct LocaleManager {
    pub current_language: LanguageIdentifier,
}

impl Global for LocaleManager {}

impl LocaleManager {
    pub fn new() -> Self {
        Self {
            current_language: langid!("ru"),
        }
    }

    pub fn translate(&self, key: &str) -> String {
        LOCALES.try_lookup(&self.current_language, key)
            .unwrap_or_else(|| {
                println!("[Localization Warning] Key '{}' not found!", key);
                key.to_string() 
            })
    }

    pub fn toggle_language(&self, cx: &mut gpui::App) {
        let current_manager = cx.global::<Self>().clone();
        
        let new_lang = if current_manager.current_language == langid!("ru") {
            langid!("en")
        } else {
            langid!("ru")
        };

        cx.set_global(LocaleManager { current_language: new_lang });
        cx.refresh_windows();
    }
}

pub fn tr(cx: &gpui::App, key: &str) -> gpui::SharedString {
    cx.global::<LocaleManager>().translate(key).into()
}