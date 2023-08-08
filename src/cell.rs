//! Allow your users to perform actions by pressing a button.
//!
//! A [`CellWidget`] has some local [`State`].
use iced::alignment;
use iced::event;
use iced::advanced::layout;
use iced::advanced::mouse;
use iced::advanced::renderer;
use iced::advanced::widget::tree;
use iced::widget::text;
use iced::widget::button::{StyleSheet};

pub struct CellWidget<Message, Renderer = iced::Renderer>
where Renderer: iced::advanced::Renderer, Renderer::Theme: StyleSheet {
  pub content: String,
  pub size: f32,
  pub width: iced::Length,
  pub height: iced::Length,
  pub padding: iced::Padding,
  pub style: <Renderer::Theme as StyleSheet>::Style,
  pub on_left_click: Option<Message>,
  pub on_right_click: Option<Message>,
  pub on_press: Option<Message>,
  pub on_release: Option<Message>,
}

impl Default for CellWidget<crate::Message, iced::Renderer> {
  fn default() -> Self {
    CellWidget {
      content: String::new(),
      size: 16.0,
      width: iced::Length::Fixed(20.0),
      height: iced::Length::Fixed(20.0),
      padding: iced::Padding::ZERO,
      style: Default::default(),
      on_left_click: None, on_right_click: None, on_press: None, on_release: None,
    }
  }
}

impl<Message, Renderer> iced::advanced::Widget<Message, Renderer> for CellWidget<Message, Renderer>
where
  Message: Clone,
  Renderer: iced::advanced::text::Renderer,
  Renderer::Theme: StyleSheet,
{

  fn state(&self) -> tree::State {
    tree::State::new(State::new())
  }
    
  fn width(&self) -> iced::Length {
    self.width
  }

  fn height(&self) -> iced::Length {
    self.height
  }

  fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
    let limits = limits.width(self.width).height(self.height);
    layout::Node::new(limits.fill())
  }

  fn on_event(&mut self, tree: &mut tree::Tree, event: event::Event, layout: iced::advanced::Layout<'_>, cursor: mouse::Cursor,
    _renderer: &Renderer, _clipboard: &mut dyn iced::advanced::Clipboard, shell: &mut iced::advanced::Shell<'_, Message>, _viewport: &iced::Rectangle,
  ) -> event::Status {
    
    match event {
      event::Event::Mouse(mouse::Event::ButtonPressed(_)) => {
        if cursor.is_over(layout.bounds()) {
          let state = tree.state.downcast_mut::<State>();
          state.is_pressed = true;
          if let Some(on_press) = &self.on_press {
            shell.publish(on_press.clone());
            return event::Status::Captured;
          }
        }
        event::Status::Ignored
      },
      event::Event::Mouse(mouse::Event::ButtonReleased(button)) => {
        let state = tree.state.downcast_mut::<State>();
        if !state.is_pressed {
          return event::Status::Ignored;
        }
        state.is_pressed = false;
        if let Some(on_release) = &self.on_release {
          shell.publish(on_release.clone());
        }
        let on_click = match button {
          mouse::Button::Left => &self.on_left_click,
          mouse::Button::Right => &self.on_right_click,
          _ => return event::Status::Captured,
        };
        if let Some(on_click) = on_click.clone() {
          if cursor.is_over(layout.bounds()) {
            shell.publish(on_click);
          }
        }
        event::Status::Captured
      },
      _ => event::Status::Ignored,
    }
    
  }

  fn draw(&self, tree: &tree::Tree, renderer: &mut Renderer, theme: &Renderer::Theme, _style: &renderer::Style, layout: iced::advanced::Layout<'_>, cursor: mouse::Cursor,_viewport: &iced::Rectangle) {
    let bounds = layout.bounds();

    let styling = if !self.on_left_click.is_some() {
      theme.disabled(&self.style)
    } else if cursor.is_over(bounds) {
      let state = tree.state.downcast_ref::<State>();
      if state.is_pressed {
        theme.pressed(&self.style)
      } else {
        theme.hovered(&self.style)
      }
    } else {
      theme.active(&self.style)
    };

    if styling.background.is_some() || styling.border_width > 0.0 {
      renderer.fill_quad(
        renderer::Quad {
          bounds,
          border_radius: styling.border_radius,
          border_width: styling.border_width,
          border_color: styling.border_color,
        },
        styling.background.unwrap_or(iced::Background::Color(iced::Color::TRANSPARENT)),
      );
    }

    let x = bounds.x + self.padding.left;
    let y = bounds.y + self.padding.right;

    renderer.fill_text(iced::advanced::Text {
        content: &self.content,
        size: self.size,
        line_height: text::LineHeight::default(),
        bounds: iced::Rectangle { x, y, ..bounds },
        color: styling.text_color,
        font: renderer.default_font(),
        horizontal_alignment: alignment::Horizontal::Left,
        vertical_alignment: alignment::Vertical::Top,
        shaping: iced::widget::text::Shaping::Advanced,
    });
    
  }

  fn mouse_interaction(&self, _tree: &tree::Tree, layout: iced::advanced::Layout<'_>, cursor: mouse::Cursor, _viewport: &iced::Rectangle, _renderer: &Renderer) -> mouse::Interaction {
    let is_mouse_over = cursor.is_over(layout.bounds());
    let is_enabled = self.on_left_click.is_some();
    if is_mouse_over && is_enabled {
      mouse::Interaction::Pointer
    } else {
      mouse::Interaction::default()
    }
  }

}

impl<'a, Message, Renderer> From<CellWidget<Message, Renderer>>
  for iced::Element<'a, Message, Renderer>
where
  Message: Clone + 'a,
  Renderer: iced::advanced::text::Renderer + 'a,
  Renderer::Theme: StyleSheet,
{
  fn from(button: CellWidget<Message, Renderer>) -> Self {
    Self::new(button)
  }
}

/// The local state of a [`CellWidget`].
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
