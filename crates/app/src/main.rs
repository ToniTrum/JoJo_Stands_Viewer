use gpui::*;
use gpui_component::Theme;

use di::AppContainer;
use ui::MainScreenEntity;
use ui::MainScreen;

fn main() {
    let base_dir = std::env::current_dir().unwrap();

    let app_container = AppContainer::new(&base_dir).unwrap();
    let stand_service = app_container.stand_service.clone();

    Application::new().run(move |cx: &mut App| {
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