use gpui::{
    SharedString, IntoElement, ElementId, ClickEvent,
    Window, App, Stateful, Div, RenderOnce, InteractiveElement,
    Styled, StatefulInteractiveElement, ParentElement,
    div, svg, px, rgb,
    prelude::FluentBuilder
};

use di::DependencyInjector;
use crate::Theme;

/// Defines the visual and structural payload inside the custom UI button.
pub enum ButtonContentType {
    Text(SharedString),
    Icon(String, f32),
}

/// A highly reusable, composable, and customizable stateful component built on top of GPUI primitives.
///
/// It supports fluid functional styling adjustments via custom modifier closures and lifecycle interaction event hooks.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    content_type: ButtonContentType,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    modifier: Option<Box<dyn FnOnce(Stateful<Div>) -> Stateful<Div> + 'static>>
}

impl Button {
    /// Registers an asynchronous interaction callback executed whenever the user dispatches a pointer click event.
    ///
    /// # Arguments
    ///
    /// * `on_click` - A closure triggered on execution, mutating the parent frame architecture states.
    ///
    /// # Returns
    ///
    /// * The updated `Button` builder instance.
    pub fn on_click(mut self, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(on_click));
        self
    }

    /// Appends external decorative style overrides onto the baseline layout wrapper tree state.
    ///
    /// # Arguments
    ///
    /// * `modifier` - A closure taking ownership of the existing stateful element, returning the stylized version.
    ///
    /// # Returns
    ///
    /// * The updated `Button` builder instance.
    pub fn style_modifier(mut self, modifier: impl FnOnce(Stateful<Div>) -> Stateful<Div> + 'static) -> Self {
        self.modifier = Some(Box::new(modifier));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let path_manager = cx.global::<DependencyInjector>().path_manager();
        let theme = cx.global::<Theme>();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .rounded_md()
            .p_2()
            .when_some(self.modifier, |this, modifier| {
                modifier(this)
            })
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, window, cx| (on_click)(evt, window, cx))
            })
            .child(match self.content_type {
                ButtonContentType::Text(text) => div().child(text).into_any_element(),
                ButtonContentType::Icon(icon, size) => {
                    let path = path_manager
                        .icon_path(&icon)
                        .to_string_lossy()
                        .into_owned();

                    svg()
                        .size(px(size))
                        .path(path)
                        .text_color(rgb(theme.text_color))
                        .into_any_element()
                },
            })
    }
}

/// A global functional initialization helper used to generate standalone standard interactive `Button` primitives.
///
/// # Arguments
///
/// * `id` - A unique identifier token compatible with GPUI element state matching rules.
/// * `content_type` - The state token describing what information payload the button displays internally.
///
/// # Returns
///
/// * A baseline configured `Button` component struct instance.
pub fn button(id: impl Into<ElementId>, content_type: ButtonContentType) -> Button {
    Button { 
        id: id.into(),
        content_type,
        on_click: None,
        modifier: None
    }
}