//! Allow your users to perform actions by pressing a button.
//!
//! A [`CellWidget`] has some local [`State`].
use iced::alignment;
use iced::event;
use iced::advanced::layout;
use iced::advanced::mouse;
use iced::advanced::renderer;
use iced::advanced::widget::tree;
use iced::widget::button;
use iced::widget::text as widget_text;
use iced::advanced::text as advanced_text;

pub struct Cell<Message> {
  pub content: char,
  pub size: u8,
  pub length: u8,
  pub padding: iced::Padding,
  pub revealed: bool,
  pub color: iced::Color,
  pub on_left_click: Option<Message>,
  pub on_middle_click: Option<Message>,
  pub on_right_click: Option<Message>,
  pub on_press: Option<Message>,
  pub on_release: Option<Message>,
}

impl Default for Cell<crate::Message> {
  fn default() -> Self {
    Cell {
      content: ' ',
      size: 16,
      length: 20,
      padding: iced::Padding::ZERO,
      color: iced::Color::WHITE,
      revealed: false,
      on_left_click: None, on_middle_click: None, on_right_click: None, on_press: None, on_release: None,
    }
  }
}

impl<Message> iced::advanced::Widget<Message, iced::Renderer> for Cell<Message>
where Message: Clone
{
  fn state(&self) -> tree::State {
    tree::State::new(State::new())
  }
    
  fn width(&self) -> iced::Length {
    iced::Length::Fixed(self.length as f32)
  }

  fn height(&self) -> iced::Length {
    iced::Length::Fixed(self.length as f32)
  }

  fn layout(&self, _renderer: &iced::Renderer, limits: &layout::Limits) -> layout::Node {
    let limits = limits.width(iced::Length::Fixed(self.length as f32)).height(iced::Length::Fixed(self.length as f32));
    layout::Node::new(limits.fill())
  }

  fn on_event(&mut self, tree: &mut tree::Tree, event: event::Event, layout: iced::advanced::Layout<'_>, cursor: mouse::Cursor,
    _renderer: &iced::Renderer, _clipboard: &mut dyn iced::advanced::Clipboard, shell: &mut iced::advanced::Shell<'_, Message>, _viewport: &iced::Rectangle,
  ) -> event::Status {
    
    match event {
      event::Event::Mouse(mouse::Event::ButtonPressed(button)) => {
        if cursor.is_over(layout.bounds()) {
          let state = tree.state.downcast_mut::<State>();
          match button {
            mouse::Button::Left => state.is_left_pressed = true,
            mouse::Button::Right => state.is_right_pressed = true,
            _ => {state.is_left_pressed = true; state.is_right_pressed = true},
          };
          if let Some(on_press) = &self.on_press {
            shell.publish(on_press.clone());
            return event::Status::Captured;
          }
        }
        event::Status::Ignored
      },
      event::Event::Mouse(mouse::Event::ButtonReleased(_)) => {
        let state = tree.state.downcast_mut::<State>();
        
        //If both buttons are pressed, then unpressing either one will trigger a "middle click" event.
        let on_click = match (state.is_left_pressed, state.is_right_pressed) {
          (true, false) => &self.on_left_click,
          (false, true) => &self.on_right_click,
          (true, true) => &self.on_middle_click,
          (false, false) => return event::Status::Ignored,
        };
        state.is_left_pressed = false;
        state.is_right_pressed = false;

        if let Some(on_release) = &self.on_release {
          shell.publish(on_release.clone());
        }
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

  fn draw(&self, tree: &tree::Tree, renderer: &mut iced::Renderer, theme: &iced::Theme, _style: &renderer::Style, layout: iced::advanced::Layout<'_>, cursor: mouse::Cursor,_viewport: &iced::Rectangle) {
    let bounds = layout.bounds();
    
    if !self.revealed {
      let style: iced::theme::Button = Default::default();

      let styling = if !self.on_left_click.is_some() {
        button::StyleSheet::disabled(theme, &style)
      } else if cursor.is_over(bounds) {
        let state = tree.state.downcast_ref::<State>();
        match state.is_left_pressed || state.is_right_pressed {
          true => button::StyleSheet::pressed(theme, &style),
          false => button::StyleSheet::hovered(theme, &style),
        }
      } else {
        button::StyleSheet::active(theme, &style)
      };

      if styling.background.is_some() || styling.border_width > 0.0 {
        iced::advanced::Renderer::fill_quad(renderer,
          renderer::Quad {
            bounds,
            border_radius: styling.border_radius,
            border_width: styling.border_width,
            border_color: styling.border_color,
          },
          styling.background.unwrap_or(iced::Background::Color(iced::Color::TRANSPARENT)),
        );
      }
    } else if self.revealed {
      iced::advanced::Renderer::fill_quad(renderer,
        renderer::Quad {
          bounds,
          border_radius: 0.0.into(),
          border_width: 0.0.into(),
          border_color: iced::Color::WHITE,
        },
        iced::Background::Color(iced::Color::WHITE)
      );
    }

    advanced_text::Renderer::fill_text(renderer, iced::advanced::Text {
        content: &self.content.to_string(),
        size: self.size as f32,
        line_height: widget_text::LineHeight::default(),
        bounds: iced::Rectangle {
          x: bounds.x + self.padding.left, 
          y: bounds.y + self.padding.top,
          ..bounds
        },
        color: self.color,
        font: iced::Font::MONOSPACE,
        horizontal_alignment: alignment::Horizontal::Left,
        vertical_alignment: alignment::Vertical::Top,
        shaping: widget_text::Shaping::Advanced,
    });
    
  }

  fn mouse_interaction(&self, _tree: &tree::Tree, layout: iced::advanced::Layout<'_>, cursor: mouse::Cursor, _viewport: &iced::Rectangle, _renderer: &iced::Renderer) -> mouse::Interaction {
    let is_mouse_over = cursor.is_over(layout.bounds());
    let is_enabled = self.on_left_click.is_some();
    if is_mouse_over && is_enabled {
      mouse::Interaction::Pointer
    } else {
      mouse::Interaction::default()
    }
  }

}

impl<'a, Message> From<Cell<Message>> for iced::Element<'a, Message>
where Message: Clone + 'a
{
  fn from(button: Cell<Message>) -> Self {
    Self::new(button)
  }
}

/// For middle press, both left and right buttons get set to true
#[derive(Clone)]
pub struct State {
  is_left_pressed: bool,
  is_right_pressed: bool,
}

impl State {
  /// Creates a new [`State`].
  pub fn new() -> State {
    State {
      is_left_pressed: false,
      is_right_pressed: false
    }
  }
}
