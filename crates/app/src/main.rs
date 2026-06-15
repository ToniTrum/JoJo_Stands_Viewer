use gpui::*;
use std::borrow::Cow;
use anyhow::Result;

use di::DependencyInjector;
use ui::MainScreenEntity;
use ui::MainScreen;
use ui::themes::Theme;

struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

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

        let window_entity = cx.new(|_cx| {
            MainScreenEntity::new(stand_service)
        });

        cx.open_window(
            gpui::WindowOptions::default(),
            |_, cx| {
                cx.new(|cx| {
                    MainScreen::new(window_entity, cx)
                })
            },
        ).unwrap();
    });
}