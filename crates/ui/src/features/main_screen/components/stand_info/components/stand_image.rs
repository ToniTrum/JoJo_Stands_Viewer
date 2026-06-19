use gpui::{
    IntoElement, RenderOnce, Window, App, 
    ObjectFit, Styled, StyledImage,
    img
};

use di::DependencyInjector;

/// A lightweight visual component responsible for dynamically loading and rendering a Stand's profile image.
#[derive(IntoElement)]
pub struct StandImage {
    stand_name: String
}

impl StandImage {
    /// Constructs a new `StandImage` component instance for a specific Stand entity.
    ///
    /// # Arguments
    ///
    /// * `stand_name` - The unique string identifier name of the targeted Stand.
    ///
    /// # Returns
    ///
    /// * An initialized `StandImage` layout node builder.
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
