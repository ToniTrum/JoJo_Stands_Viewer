use gpui::*;

use super::MainScreenEntity;
use super::components::Sidebar;

pub struct MainScreen {
    entity: Entity<MainScreenEntity>,
    _subscription: Subscription
}

impl MainScreen {
    pub fn new(entity: Entity<MainScreenEntity>, cx: &mut Context<Self>) -> Self {
        let _subscription = cx.observe(&entity, |_, _, cx| cx.notify());
        MainScreen {
            entity,
            _subscription
        }
    }
}

impl Render for MainScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity_state = self.entity.read(cx);
        let stands = entity_state.stands();
        let entity_clone = self.entity.clone();

        div()
            .size_full()
            .flex()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child(
                Sidebar::new(stands.to_vec(), move |stand_name, _window, cx| {
                    entity_clone.update(cx, |entity, _cx| {
                        entity.select_stand(stand_name); 
                });
            }))
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .child("JoJo Stands List")
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .child("Select a stand from the sidebar")
                            )
                    )
            )
    }
}