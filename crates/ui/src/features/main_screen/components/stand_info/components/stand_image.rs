use gpui::*;

use di::DependencyInjector;

#[derive(IntoElement)]
pub struct StandImage {
    stand_name: String
}

impl StandImage {
    pub fn new(stand_name: String) -> Self {
        Self {
            stand_name
        }
    }
}

impl RenderOnce for StandImage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let di = cx.global::<DependencyInjector>();
        let path_manager = di.path_manager();

        let image_path = path_manager.image_path(&format!("{}.png", self.stand_name));

        img(image_path)
            .size_full()
            .object_fit(ObjectFit::Contain)
    }
}
