use gpui::{
    Entity, Subscription, Render, Window, Context,
    IntoElement, Styled, ParentElement,
    div, rgb
};

use super::MainScreenState;
use super::components::Sidebar;
use super::components::StandInfo;
use crate::Theme;

/// The UI view container component representing the main screen.
pub struct MainScreen {
    entity: Entity<MainScreenState>,
    _subscription: Subscription
}

impl MainScreen {
    /// Constructs a new `MainScreen` instance and binds a change observation listener.
    ///
    /// # Arguments
    ///
    /// * `entity` - An encapsulated GPUI `Entity` holding the reactive model state data logic.
    /// * `cx` - A mutable reference to the view's current operational execution context.
    ///
    /// # Returns
    ///
    /// * An initialized view layout node subscribing to state events.
    pub fn new(entity: Entity<MainScreenState>, cx: &mut Context<Self>) -> Self {
        let _subscription = cx.observe(&entity, |_, _, cx| cx.notify());
        MainScreen {
            entity,
            _subscription
        }
    }
}

impl Render for MainScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.entity.read(cx);
        let stands = state.stands();
        let state_clone = self.entity.clone();
        let selected_stand = state.selected_stand();

        let theme = cx.global::<Theme>();

        div()
            .size_full()
            .flex()
            .bg(rgb(theme.background_color))
            .text_color(rgb(theme.text_color))
            .child(
                Sidebar::new(stands.to_vec(), move |stand_name, _window, cx| {
                    state_clone.update(cx, |state, _cx| {
                        state.select_stand(stand_name); 
                    });
                })
            )
            .child(
                StandInfo::new(selected_stand)
            )
    }
}