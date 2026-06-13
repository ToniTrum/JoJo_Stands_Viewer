use gpui::*;
use std::f32::consts::{PI, FRAC_PI_2};

use core::models::StandModel;

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
                .text_color(rgb(0xaa0000))
                .text_size(px(20.0))
                .child("No stand selected");
        };
        
        let _stat_labels = ["Power", "Speed", "Range", "Power Persistence", "Precision", "Development Potential"];
        let stats = [
            stand.power().to_u8(),
            stand.speed().to_u8(),
            stand.range().to_u8(),
            stand.power_persistence().to_u8(),
            stand.precision().to_u8(),
            stand.development_potential().to_u8()
        ];

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
                div()
                    .flex_1()
                    .w_full()
                    .child(
                        canvas(
                            |_, _, _| {},
                            move |bounds, _, window, _cx,| {
                                let center_x = f32::from(bounds.origin.x) + f32::from(bounds.size.width) / 2.0;
                                let center_y = f32::from(bounds.origin.y) + f32::from(bounds.size.height) / 2.0;
                                let max_radius = f32::from(bounds.size.width.min(bounds.size.height)) / 2.0 - 40.0;

                                let num_axes = 6;
                                let angle_step = (2.0 * PI) / num_axes as f32;

                                for level in 0..=7 {
                                    let level_radius = max_radius * (level as f32 / 7.0);

                                    let start_angle = -FRAC_PI_2;
                                    let start_x = center_x + level_radius * start_angle.cos();
                                    let start_y = center_y + level_radius * start_angle.sin();
                                    let start_point = point(px(start_x), px(start_y));

                                    let mut path = Path::new(start_point);

                                    for i in 1..num_axes {
                                        let angle = (i as f32 * angle_step) - FRAC_PI_2;
                                        let x = center_x + level_radius * angle.cos();
                                        let y = center_y + level_radius * angle.sin();

                                        path.line_to(point(px(x), px(y)));
                                    }
                                    
                                    path.line_to(start_point);

                                    window.paint_path(path, rgb(0x3c3c3c));
                                }

                                for i in 0..num_axes {
                                    let angle = (i as f32 * angle_step) - FRAC_PI_2;
                                    let x = center_x + max_radius * angle.cos();
                                    let y = center_y + max_radius * angle.sin();

                                    let mut axis_path = Path::new(point(px(center_x), px(center_y)));
                                    axis_path.line_to(point(px(x), px(y)));

                                    window.paint_path(axis_path, rgb(0x3c3c3c));
                                }

                                let start_angle = -FRAC_PI_2;
                                let start_radius = max_radius * (stats[0] as f32 / 7.0);
                                let start_x = center_x + start_radius * start_angle.cos();
                                let start_y = center_y + start_radius * start_angle.sin();
                                let stand_start_point = point(px(start_x), px(start_y));

                                let mut stand_path = Path::new(stand_start_point);

                                for i in 1..num_axes {
                                    let angle = (i as f32 * angle_step) - FRAC_PI_2;
                                    let stat_value = stats[i] as f32;
                                    let current_radius = max_radius * (stat_value / 7.0);

                                    let x = center_x + current_radius * angle.cos();
                                    let y = center_y + current_radius * angle.sin();
                                    stand_path.line_to(point(px(x), px(y)));
                                }

                                stand_path.line_to(stand_start_point);

                                window.paint_path(stand_path.clone(), hsla(0.75, 0.6, 0.5, 0.4));
                                window.paint_path(stand_path, rgb(0xa855f7));
                            }
                        )
                        .size_full()
                    )
            )
    }
}