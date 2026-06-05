use gpui::*;

use super::MainWindowEntity;

pub struct MainWindow {
    entity: Entity<MainWindowEntity>,
    _subscription: Subscription
}

impl MainWindow {
    pub fn new(entity: Entity<MainWindowEntity>, cx: &mut Context<Self>) -> Self {
        let _subscription = cx.observe(&entity, |_, _, cx| cx.notify());
        MainWindow {
            entity,
            _subscription
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity_state = self.entity.read(cx);
        let stands = entity_state.stands();

        div()
            .size_full()
            .flex()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .w(px(250.0))
                    .h_full()
                    .bg(rgb(0x252526))
                    .border_color(rgb(0x3c3c3c))
                    .p_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .child("Stands List")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .children(stands.iter().map(|stand| {
                                        div()
                                            .p_2()
                                            .bg(rgb(0x2d2d2d))
                                            .child(stand.name().to_string())
                                    }))
                            )
                    )
            )
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