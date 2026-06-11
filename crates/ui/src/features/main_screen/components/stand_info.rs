use gpui::*;

use core::models::StandModel;

#[derive(IntoElement)]
pub struct StandInfo {
    stand: StandModel
}

impl StandInfo {
    pub fn new(stand: StandModel) -> Self {
        StandInfo {
            stand
        }
    }
}

impl RenderOnce for StandInfo {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        
        div()
            .text_color(rgb(0xffffff))
            .text_size(px(16.0))
    }
}