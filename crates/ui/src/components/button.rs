use gpui::{*, prelude::FluentBuilder};

use di::DependencyInjector;
use crate::themes::Theme;

pub enum ButtonContentType {
    Text(SharedString),
    Icon(String),
}

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    content_type: ButtonContentType,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    modifier: Option<Box<dyn FnOnce(Stateful<gpui::Div>) -> Stateful<gpui::Div> + 'static>>
}

impl Button {
    pub fn new(id: impl Into<ElementId>, content_type: ButtonContentType) -> Self {
        Button { 
            id: id.into(),
            content_type,
            on_click: None,
            modifier: None
        }
    }

    pub fn on_click(mut self, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(on_click));
        self
    }

    pub fn style_modifier(mut self, modifier: impl FnOnce(Stateful<gpui::Div>) -> Stateful<gpui::Div> + 'static) -> Self {
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
                ButtonContentType::Icon(icon) => {
                    let path = path_manager
                        .icon_path(&icon)
                        .to_string_lossy()
                        .into_owned();

                    svg()
                        .size(px(30.0))
                        .path(path)
                        .text_color(rgb(theme.text_color))
                        .into_any_element()
                },
            })
    }
}