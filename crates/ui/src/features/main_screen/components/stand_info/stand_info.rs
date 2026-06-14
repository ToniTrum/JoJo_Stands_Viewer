use gpui::*;
use gpui_component::{v_flex, scroll::ScrollableElement};

use core::models::{StandModel, RankModel};
use super::components::RadarChart;
use super::components::StandImage;

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

    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let stand = self.stand.clone().unwrap_or_default();

        let mut stand_image = div().child(StandImage::new(stand.name().to_string()));
        let mut radar_chart = div().child(RadarChart::new(
            7,
            6,
            vec![
                RankModel::E.to_string(),
                RankModel::D.to_string(),
                RankModel::C.to_string(),
                RankModel::B.to_string(),
                RankModel::A.to_string(),
            ],
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
            0x9d9d9d,
            0xff830f,
            120
        ));

        let window_width = window.bounds().size.width.to_f64() as f32;
        let sidebar_width =  250.0;
        let is_compact = window_width < 850.0;

        if is_compact {
            stand_image = stand_image
                .max_w(px(window_width - sidebar_width - 10.0))
                .h(px(350.0))
                .flex()
                .items_center()
                .justify_center();
            radar_chart = radar_chart
                .h(px(350.0))
                .mb_16()
        } else {
            stand_image = stand_image
                .w(px((window_width - sidebar_width) / 2.0 - 5.0))
                .flex()
                .items_center()
                .justify_center();
            radar_chart = radar_chart
                .w_full()
                .h_full();
        }

        let content_container = div();
        let content_container = if is_compact {
            v_flex()
                .id("stand_info")
                .flex()
                .flex_col()
                .justify_center()
                .items_center()
                .gap_2()
                .size_full()
                .overflow_y_scrollbar()
                .child(stand_image)
                .child(radar_chart)
                .into_any_element()
        } else {
            content_container
                .flex()
                .flex_row()
                .justify_center()
                .items_center()
                .gap_2()
                .size_full()
                .child(stand_image)
                .child(radar_chart)
                .into_any_element()
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
                content_container
            )
    }
}