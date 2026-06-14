use std::sync::Arc;
use gpui::*;
use gpui_component::v_flex;

use core::models::StandModel;
use crate::components::{Button, ButtonContentType};

#[derive(IntoElement)]
pub struct Sidebar {
    stands: Vec<StandModel>,
    on_stand_selected: Arc<dyn Fn(String, &mut Window, &mut App) + 'static>,
}

impl Sidebar {
    pub fn new(
        stands: Vec<StandModel>, 
        on_stand_selected: impl Fn(String, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self { 
            stands, 
            on_stand_selected: Arc::new(on_stand_selected) 
        }
    }

    pub fn _stands(&self) -> &Vec<StandModel> {
        &self.stands
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let stands = self.stands;

        div()
            .w(px(250.0))
            .h_full()
            .flex()
            .flex_col()
            .p_2()
            .gap_2()
            .justify_between()
            .child("Stand list")
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .child(
                        v_flex()
                            .id("stand_list")
                            .overflow_y_scroll()
                            .gap_2()
                            .children(stands.into_iter().enumerate().map(|(i, stand)| {
                                let stand_name = stand.name().to_string();
                                let current_name = stand_name.clone();
                                let callback_clone = self.on_stand_selected.clone();

                                Button::new(
                                    ("stand_button", i), 
                                    ButtonContentType::Text(stand_name.into())
                                )
                                .style_modifier(|style| {
                                    style
                                        .w_full()
                                        .h(px(40.0))
                                        .mb_2()
                                        .justify_start()
                                        .p_2()
                                        .bg(rgb(0x2d2d2d))
                                        .hover(|s| s.bg(rgb(0x37373d)))
                                })
                                .on_click(move |_, window, cx| {
                                    callback_clone(current_name.clone(), window, cx)
                                })
                            }))
                    )
            )
    }
}
