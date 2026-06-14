use gpui::*;

use core::models::StandModel;
use super::components::RadarChart;

#[derive(IntoElement)]
pub struct StandInfo {
    stand: Option<StandModel>,
}

impl StandInfo {
    pub fn new(stand: Option<StandModel>) -> Self {
        Self { stand }
    }
}

impl RenderOnce for StandInfo {

    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let Some(stand) = self.stand else {
            return div()
                .flex_1()
                .h_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(rgb(0xffffff))
                .text_size(px(20.0))
                .child("No stand selected");
        };

        div()
            .flex_1()
            .h_full()
            .bg(rgb(0x1e1e1e))
            .p_6()
            .flex()
            .flex_col()
            .child(
                div()
                    .text_size(px(32.0))
                    .font_weight(FontWeight::BOLD)
                    .mb(px(20.0))
                    .child(stand.name().to_string())
            )
            .child(
                RadarChart::new(
                    7,
                    6,
                    vec![
                        stand.power().to_u8() as u32,
                        stand.speed().to_u8() as u32,
                        stand.range().to_u8() as u32,
                        stand.power_persistence().to_u8() as u32,
                        stand.precision().to_u8() as u32,
                        stand.development_potential().to_u8() as u32
                    ],
                    vec![
                        String::from("Power"),
                        String::from("Speed"), 
                        String::from("Range"), 
                        String::from("Power Persistence"), 
                        String::from("Precision"), 
                        String::from("Development Potential")
                    ],
                    0x3c3c3c,
                    0xff830f,
                    50
                )
            )
    }
}