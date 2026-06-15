use gpui::*;
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(IntoElement)]
pub struct RadarChart {
    grid_levels: usize,
    num_axes: usize,
    grid_values: Vec<String>,
    stats: Vec<u32>,
    stat_labels: Vec<String>,
    grid_color: u32,
    polygon_color: u32,
    polygon_opacity: u8,
    text_color: Hsla
}

impl RadarChart {
    pub fn new(
        grid_levels: usize, 
        num_axes: usize,
        grid_values: Vec<String>,
        stats: Vec<u32>, 
        stat_labels: Vec<String>,
        grid_color: u32,
        polygon_color: u32,
        polygon_opacity: u8,
        text_color: Hsla
    ) -> Self {
        Self {
            grid_levels,
            num_axes,
            grid_values,
            stats,
            stat_labels,
            grid_color,
            polygon_color,
            polygon_opacity,
            text_color
        }
    }

    fn rgb_to_rgba(&self, color: u32, opacity: u8) -> Rgba {
        rgba((color << 8) | (opacity as u32))
    }

    fn build_grid(
        &self, 
        window: &mut Window,
        max_radius: f32,
        center_x: f32,
        center_y: f32,
        angle_step: f32
    ) {
        for level in 1..self.grid_levels {
            let level_radius = max_radius * (level as f32 / (self.grid_levels - 1) as f32);
            let start_angle = -FRAC_PI_2;
            let start_x = center_x + level_radius * start_angle.cos();
            let start_y = center_y + level_radius * start_angle.sin();

            let mut builder = PathBuilder::stroke(px(1.0));
            builder.move_to(point(px(start_x), px(start_y)));

            for i in 1..self.num_axes {
                let angle = (i as f32 * angle_step) - FRAC_PI_2;
                let x = center_x + level_radius * angle.cos();
                let y = center_y + level_radius * angle.sin();

                builder.line_to(point(px(x), px(y)));
            }
            builder.close();

            if let Ok(path) = builder.build() {
                window.paint_path(path, rgb(self.grid_color));
            }
        }
    }

    fn build_axes(
        &self, 
        window: &mut Window,
        max_radius: f32,
        center_x: f32,
        center_y: f32,
        angle_step: f32
    ) {
        for i in 0..self.num_axes {
            let angle = (i as f32 * angle_step) - FRAC_PI_2;
            let x = center_x + max_radius * angle.cos();
            let y = center_y + max_radius * angle.sin();

            let mut builder = PathBuilder::stroke(px(1.5));

            builder.move_to(point(px(center_x), px(center_y)));
            builder.line_to(point(px(x), px(y)));
            builder.close();

            if let Ok(path) = builder.build() {
                window.paint_path(path, rgb(self.grid_color));
            }
        }
    }

    fn build_polygon(
        &self, 
        window: &mut Window,
        max_radius: f32,
        center_x: f32,
        center_y: f32,
        angle_step: f32
    ) {
        let mut points = Vec::with_capacity(self.num_axes);
        for i in 0..self.num_axes {
            let angle = (i as f32 * angle_step) - FRAC_PI_2;
            let stat_value = *self.stats.get(i).unwrap_or(&0) as f32;
            let radius = max_radius * (stat_value / (self.grid_levels - 1) as f32);

            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();

            points.push(point(px(x), px(y)));
        }

        let mut builder = PathBuilder::fill();

        builder.move_to(points[0]);
        for p in points.iter().skip(1) {
            builder.line_to(*p);
        }
        builder.close();

        if let Ok(fill_path) = builder.build() {
            window.paint_path(
                fill_path,
                self.rgb_to_rgba(self.polygon_color, self.polygon_opacity)
            );
        }

        let mut builder = PathBuilder::stroke(px(2.0));

        builder.move_to(points[0]);
        for p in points.iter().skip(1) {
            builder.line_to(*p);
        }
        builder.close();

        if let Ok(stroke_path) = builder.build() {
            window.paint_path(stroke_path, rgb(self.polygon_color));
        }
    }

    fn build_labels(
        &self,
        window: &mut Window,
        max_radius: f32,
        center_x: f32,
        center_y: f32,
        angle_step: f32
    ) {
        for (i, label) in self.stat_labels.iter().enumerate() {
            if i >= self.num_axes { break; }

            let angle = (i as f32 * angle_step) - FRAC_PI_2;
            let text_radius = max_radius + 15.0;
            let x = center_x + text_radius * angle.cos();
            let y = center_y + text_radius * angle.sin();

            let text_style = TextStyle {
                font_size: px(12.0).into(),
                color: self.text_color,
                ..Default::default()
            };
            let text_layout = window.text_system().layout_line(
                label.as_str(),
                text_style.font_size.to_pixels(px(1.0)),
                &[],
                None
            );

            let text_width: f32 = text_layout.width.into();
            let text_height: f32 = text_style.font_size.to_pixels(window.rem_size()).into();

            let mut draw_x = x;
            let mut draw_y = y;

            let cos_a = angle.cos();

            if cos_a.abs() < 0.1 {
                    draw_x -= text_width / 2.0;
            } else if cos_a < 0.0 {
                draw_x -= text_width;
            }
            draw_y += text_height / 4.0;

            let origin = point(px(draw_x), px(draw_y));
            for run in &text_layout.runs {
                for glyph in &run.glyphs {
                    let glyph_origin = origin + glyph.position;

                    let _ = window.paint_glyph(
                        glyph_origin, 
                        run.font_id, 
                        glyph.id, 
                        text_style.font_size.to_pixels(window.rem_size()), 
                        text_style.color
                    );
                }
            }
        }
    }

    fn build_values_points(
        &self, 
        window: &mut Window,
        max_radius: f32,
        center_x: f32,
        center_y: f32,
    ) {
        for i in 1..self.grid_values.len() {
            let radius = max_radius * (i as f32 / (self.grid_levels - 1) as f32);
            let start_angle = -FRAC_PI_2;
            let start_x = center_x - 5.0;
            let start_y = center_y + radius * start_angle.sin();

            let mut builder = PathBuilder::stroke(px(2.0));
            builder.move_to(point(px(start_x), px(start_y)));
            builder.line_to(point(px(start_x + 10.0), px(start_y)));
            builder.close();

            if let Ok(path) = builder.build() {
                window.paint_path(path, rgb(self.grid_color));
            }
        }
    }

    fn build_values_labels(
        &self, 
        window: &mut Window,
        max_radius: f32,
        center_x: f32,
        center_y: f32,
    ) {
        for (i, value) in self.grid_values.iter().enumerate() {
            let text_style = TextStyle {
                font_size: px(12.0).into(),
                color: self.text_color,
                ..Default::default()
            };
            let text_layout = window.text_system().layout_line(
                value.as_str(),
                text_style.font_size.to_pixels(px(1.0)),
                &[],
                None
            );

            let text_height: f32 = text_style.font_size.to_pixels(window.rem_size()).into();
            let radius = max_radius * ((i + 1) as f32 / (self.grid_levels - 1) as f32);
            let start_angle = -FRAC_PI_2;
            let start_x = center_x + 7.0;
            let start_y = center_y + radius * start_angle.sin() + text_height / 2.0;

            let origin = point(px(start_x), px(start_y));
            for run in &text_layout.runs {
                for glyph in &run.glyphs {
                    let glyph_origin = origin + glyph.position;

                    let _ = window.paint_glyph(
                        glyph_origin, 
                        run.font_id, 
                        glyph.id,
                        text_style.font_size.to_pixels(window.rem_size()), 
                        text_style.color
                    );
                }
            }
        }
    }
}

impl RenderOnce for RadarChart {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, _cx,| {
                        let center_x = f32::from(bounds.origin.x) + f32::from(bounds.size.width) / 2.0;
                        let center_y = f32::from(bounds.origin.y) + f32::from(bounds.size.height) / 2.0;
                        let max_radius = f32::from(bounds.size.width.min(bounds.size.height)) / 2.0 - 40.0;

                        let angle_step = (2.0 * PI) / self.num_axes as f32;

                        self.build_grid(window, max_radius, center_x, center_y, angle_step);
                        self.build_axes(window, max_radius, center_x, center_y, angle_step);
                        self.build_values_points(window, max_radius, center_x, center_y);
                        self.build_polygon(window, max_radius, center_x, center_y, angle_step);
                        self.build_values_labels(window, max_radius, center_x, center_y);
                        self.build_labels(window, max_radius, center_x, center_y, angle_step);
                    }
                )
                .size_full()
            )
    }
}