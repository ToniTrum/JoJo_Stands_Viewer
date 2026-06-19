use std::sync::Arc;
use gpui::{
    IntoElement, Window, App, RenderOnce, Styled,
    ParentElement, InteractiveElement, StatefulInteractiveElement,
    div, px, rgb,
};
use gpui_component::v_flex;

use core::models::StandModel;
use crate::shared::{button, ButtonContentType};
use crate::{Theme, locale::tr};

/// A structural navigation viewport component representing the left sidebar panel.
#[derive(IntoElement)]
pub struct Sidebar {
    stands: Vec<StandModel>,
    on_stand_selected: Arc<dyn Fn(String, &mut Window, &mut App) + 'static>,
}

impl Sidebar {
    /// Constructs a new `Sidebar` panel.
    ///
    /// # Arguments
    ///
    /// * `stands` - A vector storage payload containing the pre-cached `StandModel` entities.
    /// * `on_stand_selected` - A closure executed when an element row is clicked, forwarding the entity identifier key.
    ///
    /// # Returns
    ///
    /// * An initialized `Sidebar` layout node builder instance.
    pub fn new(
        stands: Vec<StandModel>, 
        on_stand_selected: impl Fn(String, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self { 
            stands, 
            on_stand_selected: Arc::new(on_stand_selected) 
        }
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let stands = self.stands;
        let theme = cx.global::<Theme>().clone();

        div()
            .w(px(250.0))
            .h_full()
            .flex()
            .flex_col()
            .p_2()
            .gap_2()
            .justify_between()
            .child(tr(cx, "stand_list"))
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

                                button(
                                    ("stand_button", i), 
                                    ButtonContentType::Text(stand_name.into())
                                )
                                .style_modifier(move |style| {
                                    style
                                        .w_full()
                                        .h(px(40.0))
                                        .mb_2()
                                        .justify_start()
                                        .p_2()
                                        .bg(rgb(theme.button_color))
                                        .hover(|s| s.bg(rgb(theme.button_hover_color)))
                                })
                                .on_click(move |_, window, cx| {
                                    callback_clone(current_name.clone(), window, cx)
                                })
                            }))
                    )
            )
    }
}