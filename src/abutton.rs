//! Allow your users to perform actions by pressing a button.
//!
//! A [`Button`] has some local [`State`].
use iced::event::{self, Event};
use iced::advanced::layout;
use iced::advanced::mouse;
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::widget::Operation;
use iced::{
    Background, Color, Element, Length, Padding, Point, Rectangle, Vector
};
use iced::advanced::{
    Clipboard, Layout, Shell, Widget,
};


pub use iced::widget::button::{Appearance, StyleSheet};

pub struct Button<'a, Message, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    content: Element<'a, Message, Renderer>,
    on_left_click: Option<Message>,
    on_right_click: Option<Message>,
    on_press: Option<Message>,
    on_release: Option<Message>,
    width: Length,
    height: Length,
    padding: Padding,
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> Button<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`Button`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Button {
            content: content.into(),
            on_left_click: None,
            on_right_click: None,
            on_press: None,
            on_release: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::new(5.0),
            style: <Renderer::Theme as StyleSheet>::Style::default(),
        }
    }

    /// Sets the width of the [`Button`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Button`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`Button`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the message that will be produced when the [`Button`] is left clicked.
    ///
    /// Unless `on_left_click` is called, the [`Button`] will be disabled.
    pub fn on_left_click(mut self, on_left_click: Message) -> Self {
        self.on_left_click = Some(on_left_click);
        self
    }

    /// Sets the message that will be produced when the [`Button`] is right clicked.
    pub fn on_right_click(mut self, on_right_click: Message) -> Self {
        self.on_right_click = Some(on_right_click);
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed. (With any button)
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(on_press);
        self
    }

    /// Sets the message that will be produced when the [`Button`] is released. (With any button)
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }


    /// Sets the style variant of this [`Button`].
    pub fn style(
        mut self,
        style: <Renderer::Theme as StyleSheet>::Style,
    ) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Button<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout(
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            |renderer, limits| {
                self.content.as_widget().layout(renderer, limits)
            },
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return event::Status::Captured;
        }

        update(event, layout, cursor, shell, &self.on_left_click, &self.on_right_click, &self.on_press, &self.on_release, || {
            tree.state.downcast_mut::<State>()
        })
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();

        let styling = draw(
            renderer,
            bounds,
            cursor,
            self.on_left_click.is_some(),
            theme,
            &self.style,
            || tree.state.downcast_ref::<State>(),
        );

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: styling.text_color,
            },
            content_layout,
            cursor,
            &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse_interaction(layout, cursor, self.on_left_click.is_some())
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<Button<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced::advanced::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(button: Button<'a, Message, Renderer>) -> Self {
        Self::new(button)
    }
}

/// The local state of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_pressed: bool,
}

impl State {
    /// Creates a new [`State`].
    pub fn new() -> State {
        State::default()
    }
}

/// Processes the given [`Event`] and updates the [`State`] of a [`Button`]
/// accordingly.
pub fn update<'a, Message: Clone>(
  event: Event,
  layout: Layout<'_>,
  cursor: mouse::Cursor,
  shell: &mut Shell<'_, Message>,
  on_left_click: &Option<Message>,
  on_right_click: &Option<Message>,
  on_press: &Option<Message>,
  on_release: &Option<Message>,
  state: impl FnOnce() -> &'a mut State,
) -> event::Status {
  match event {
    Event::Mouse(mouse::Event::ButtonPressed(_)) => {
      if cursor.is_over(layout.bounds()) {
        state().is_pressed = true;
        if let Some(on_press) = on_press.clone() {
          shell.publish(on_press);
          return event::Status::Captured;
        }
      }
    },
    Event::Mouse(mouse::Event::ButtonReleased(button)) => {
      let state = state();
      if !state.is_pressed {
        return event::Status::Ignored;
      }
      state.is_pressed = false;
      if let Some(on_release) = on_release.clone() {
        shell.publish(on_release);
      }
      let on_click = match button {
        mouse::Button::Left => on_left_click,
        mouse::Button::Right => on_right_click,
        _ => return event::Status::Captured,
      };
      if let Some(on_click) = on_click.clone() {
        if cursor.is_over(layout.bounds()) {
          shell.publish(on_click);
        }
      }
      return event::Status::Captured;
    },
    _ => {}
  }

  event::Status::Ignored
}

/// Draws a [`Button`].
pub fn draw<'a, Renderer: iced::advanced::Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    cursor: mouse::Cursor,
    is_enabled: bool,
    style_sheet: &dyn StyleSheet<
        Style = <Renderer::Theme as StyleSheet>::Style,
    >,
    style: &<Renderer::Theme as StyleSheet>::Style,
    state: impl FnOnce() -> &'a State,
) -> Appearance
where
    Renderer::Theme: StyleSheet,
{
    let is_mouse_over = cursor.is_over(bounds);

    let styling = if !is_enabled {
        style_sheet.disabled(style)
    } else if is_mouse_over {
        let state = state();

        if state.is_pressed {
            style_sheet.pressed(style)
        } else {
            style_sheet.hovered(style)
        }
    } else {
        style_sheet.active(style)
    };

    if styling.background.is_some() || styling.border_width > 0.0 {
        if styling.shadow_offset != Vector::default() {
            // TODO: Implement proper shadow support
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x + styling.shadow_offset.x,
                        y: bounds.y + styling.shadow_offset.y,
                        ..bounds
                    },
                    border_radius: styling.border_radius,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                Background::Color([0.0, 0.0, 0.0, 0.5].into()),
            );
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: styling.border_radius,
                border_width: styling.border_width,
                border_color: styling.border_color,
            },
            styling
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );
    }

    styling
}

/// Computes the layout of a [`Button`].
pub fn layout<Renderer>(
    renderer: &Renderer,
    limits: &layout::Limits,
    width: Length,
    height: Length,
    padding: Padding,
    layout_content: impl FnOnce(&Renderer, &layout::Limits) -> layout::Node,
) -> layout::Node {
    let limits = limits.width(width).height(height);

    let mut content = layout_content(renderer, &limits.pad(padding));
    let padding = padding.fit(content.size(), limits.max());
    let size = limits.pad(padding).resolve(content.size()).pad(padding);

    content.move_to(Point::new(padding.left, padding.top));

    layout::Node::with_children(size, vec![content])
}

/// Returns the [`mouse::Interaction`] of a [`Button`].
pub fn mouse_interaction(
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    is_enabled: bool,
) -> mouse::Interaction {
    let is_mouse_over = cursor.is_over(layout.bounds());

    if is_mouse_over && is_enabled {
        mouse::Interaction::Pointer
    } else {
        mouse::Interaction::default()
    }
}
