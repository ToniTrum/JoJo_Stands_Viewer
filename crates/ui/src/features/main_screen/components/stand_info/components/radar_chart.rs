use gpui::*;
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(IntoElement)]
pub struct RadarChart {
    grid_levels: usize,
    num_axes: usize,
    stats: Vec<u32>,
    stat_labels: Vec<SharedString>,
    grid_color: u32,
    polygon_color: u32,
    polygon_opacity: u8
}

impl RadarChart {
    pub fn new(
        grid_levels: usize, 
        num_axes: usize, 
        stats: Vec<u32>, 
        stat_labels: Vec<SharedString>,
        grid_color: u32,
        polygon_color: u32,
        polygon_opacity: u8
    ) -> Self {
        Self {
            grid_levels,
            num_axes,
            stats,
            stat_labels,
            grid_color,
            polygon_color,
            polygon_opacity
        }
    }

    fn build_grid(
        &self, 
        window: &mut Window,
        max_radius: f32,
        center_x: f32,
        center_y: f32,
        angle_step: f32
    ) {
        for level in 1..=self.grid_levels {
            let level_radius = max_radius * (level as f32 / self.grid_levels as f32);

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
            let radius = max_radius * (stat_value / 7.0);

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
                rgba((self.polygon_color << 8) | (self.polygon_opacity as u32))
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
                        self.build_polygon(window, max_radius, center_x, center_y, angle_step);
                    }
                )
                .size_full()
            )
            // .children(
            //     self.stat_labels.iter().enumerate().map(|(i, label)| {
            //         let angle_step = (2.0 * PI) / self.num_axes as f32;
            //         let angle = (i as f32 * angle_step) - FRAC_PI_2;

            //         let cos_a = angle.cos();
            //         let sin_a = angle.sin();

            //         let offset_x = cos_a * 15.0;
            //         let offset_y = sin_a * 15.0;

            //         let mut label_div = div()
            //             .absolute()
            //             .left(Length::to_pixels(&self, base_size, rem_size))
            //             .top(RelativeDist::Sub(Value::Percent(50.0), px(-sin_a * 15.0)))
            //             .text_size(px(12.0))
            //             .text_color(rgb(0x9ca3af));
            //     })
            // )
    }
}