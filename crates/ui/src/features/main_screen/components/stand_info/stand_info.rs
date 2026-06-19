use gpui::{
    IntoElement, RenderOnce, Window, App, ParentElement, 
    Styled, InteractiveElement, FontWeight,
    div, px, rgb
};
use gpui_component::{v_flex, h_flex, scroll::ScrollableElement};

use core::models::StandModel;
use core::types::Rank;
use super::components::{StandImage, RadarChart};
use crate::shared::{button, ButtonContentType};
use crate::Theme;
use crate::locale::{tr, Locale};

/// A presentation component that displays comprehensive details about a selected Stand model.
#[derive(IntoElement)]
pub struct StandInfo {
    stand: Option<StandModel>,
}

impl StandInfo {
    /// Constructs a new `StandInfo` display panel wrapping an optional `StandModel` record.
    ///
    /// # Arguments
    ///
    /// * `stand` - The optional active entity context model to display inside the layout view.
    pub fn new(stand: Option<StandModel>) -> Self {
        Self { stand }
    }
}

impl RenderOnce for StandInfo {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let stand = self.stand.clone().unwrap_or_default();
        let theme = cx.global::<Theme>().clone();
        let locale = cx.global::<Locale>().clone();

        let mut stand_image = div().child(StandImage::new(stand.name().to_string()));
        let mut radar_chart = div().child(RadarChart::new(
            7,
            6,
            vec![
                Rank::E.to_string(),
                Rank::D.to_string(),
                Rank::C.to_string(),
                Rank::B.to_string(),
                Rank::A.to_string(),
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
                tr(cx, "power").to_string(),
                tr(cx, "speed").to_string(),
                tr(cx, "range").to_string(),
                tr(cx, "power_persistence").to_string(), 
                tr(cx, "precision").to_string(),
                tr(cx, "development_potential").to_string()
            ],
            theme.grid_color,
            theme.polygon_color,
            theme.polygon_opacity,
            theme.radar_text_color
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
            .p_6()
            .flex()
            .flex_col()
            .child(
                h_flex()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .text_size(px(32.0))
                    .font_weight(FontWeight::BOLD)
                    .mb(px(20.0))
                    .w_full()
                    .child(stand.name().to_string())
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_2()
                            .child(
                                button(
                                    "language_button", 
                                    ButtonContentType::Icon(String::from("language.svg"), 30.0)
                                )
                                .style_modifier(move |style| {
                                    style
                                        .p_2()
                                        .bg(rgb(theme.button_color))
                                        .hover(|s| s.bg(rgb(theme.button_hover_color)))
                                })
                                .on_click(move |_, _window, cx| {
                                    locale.toggle_language(cx);
                                })
                            )
                            .child(
                                button(
                                    "theme_button", 
                                    ButtonContentType::Icon(String::from("theme.svg"), 30.0)
                                )
                                .style_modifier(move |style| {
                                    style
                                        .p_2()
                                        .bg(rgb(theme.button_color))
                                        .hover(|s| s.bg(rgb(theme.button_hover_color)))
                                })
                                .on_click(move |_, _window, cx| {
                                    theme.toggle_theme(cx);
                                })
                            )
                    )
            )
            .child(
                content_container
            )
    }
}