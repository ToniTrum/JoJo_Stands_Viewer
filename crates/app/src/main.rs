use gpui::*;

use di::AppContainer;
use ui::MainWindowEntity;
use ui::MainWindow;

fn main() {
    let base_dir = std::env::current_dir().unwrap();
    println!("base dir: {}", base_dir.display());

    let app_container = AppContainer::new(&base_dir).unwrap();
    let stand_service = app_container.stand_service.clone();

    Application::new().run(move |cx: &mut App| {
        let window_entity = cx.new(|_cx| {
            MainWindowEntity::new(stand_service)
        });

        cx.open_window(
            gpui::WindowOptions::default(),
            |_, cx| {
                cx.new(|cx| {
                    MainWindow::new(window_entity, cx)
                })
            },
        ).unwrap();
    });
}