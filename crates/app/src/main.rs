use gpui::*;
use gpui_component::Theme;

use di::DependencyInjector;
use ui::MainScreenEntity;
use ui::MainScreen;

fn main() {
    let base_dir = std::env::current_dir().unwrap();

    let di = DependencyInjector::init(&base_dir).unwrap();
    let stand_service = di.stand_service();

    Application::new().run(move |cx: &mut App| {
        cx.set_global(Theme::default());
        cx.set_global(di);

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