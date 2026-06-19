use gpui::{
    AssetSource, SharedString, Application, App, AppContext
};
use std::borrow::Cow;
use anyhow::Result;

use di::DependencyInjector;
use ui::{
    MainScreenState, MainScreen, Theme, locale::Locale
};

/// An application asset provider responsible for bridging GPUI's resource management layer with external storage locations.
struct Assets;

impl AssetSource for Assets {
    /// Dynamically loads a resource binary payload from the local filesystem based on its relative path string.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice indicating the unique lookup destination route of the target asset.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Cow::Owned))` containing the raw file byte array vector data if found,
    ///   `Ok(None)` if the resource file doesn't exist, or an IO `Err` if a filesystem error occurs.
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    /// Scans a directory path and indexes all nested asset identities inside it.
    ///
    /// Currently stubbed to return an empty collection since the framework accesses targeted 
    /// media assets explicitly by their exact paths via the `load` method.
    ///
    /// # Arguments
    ///
    /// * `_path` - A root directory path token slice where the recursive scanning would begin.
    ///
    /// # Returns
    ///
    /// * An empty vector array wrapped in a successful `Result`.
    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

fn main() {
    let base_dir = std::env::current_dir().unwrap();

    let di = DependencyInjector::init(&base_dir).unwrap();
    let stand_service = di.stand_service();

    Application::new().with_assets(Assets{}).run(move |cx: &mut App| {
        cx.set_global(gpui_component::Theme::default());
        cx.set_global(di);
        cx.set_global(Theme::default());
        cx.set_global(Locale::new());

        let main_state = cx.new(|_cx| {
            MainScreenState::new(stand_service)
        });

        cx.open_window(
            gpui::WindowOptions::default(),
            |_, cx| {
                cx.new(|cx| {
                    MainScreen::new(main_state, cx)
                })
            },
        ).unwrap();
    });
}