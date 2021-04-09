use std::{error::Error, io};

use termion::raw::RawTerminal;
use tui::{
  backend::TermionBackend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  text::Span,
  widgets::{Block, BorderType, Borders, Paragraph},
  Frame,
};

use super::{prompt_value, util::*};
use crate::{info::get_hostname, Greeter, Mode};

const GREETING_INDEX: usize = 0;
const USERNAME_INDEX: usize = 1;
const ANSWER_INDEX: usize = 2;
const MESSAGE_INDEX: usize = 3;

const TITLE: &str = "Authenticate into";
const USERNAME: &str = "Username:";
const WORKING: &str = "Please wait...";

pub fn draw(greeter: &mut Greeter, f: &mut Frame<TermionBackend<RawTerminal<io::Stdout>>>) -> Result<(u16, u16), Box<dyn Error>> {
  let size = f.size();

  let width = greeter.width();
  let height = get_height(&greeter);
  let container_padding = greeter.container_padding();
  let prompt_padding = 0; // greeter.prompt_padding();
  let x = (size.width - width) / 2;
  let y = (size.height - height) / 2;

  let container = Rect::new(x, y, width, height);
  let frame = Rect::new(x + container_padding, y + container_padding, width - container_padding, height - container_padding);
  let hostname = Span::from(format!(" {} {} ", TITLE, get_hostname()));
  let block = Block::default().title(hostname).borders(Borders::ALL).border_type(BorderType::Rounded);

  f.render_widget(block, container);

  let (message, message_height) = get_message_height(greeter, 1, 1);
  let (greeting, greeting_height) = get_greeting_height(greeter, 1, 0);

  let username_padding = if greeter.mode == Mode::Username && prompt_padding == 0 { 1 } else { prompt_padding };
  let answer_padding = if prompt_padding == 0 { 1 } else { prompt_padding };

  let constraints = [
    Constraint::Length(greeting_height),                                                                  // Greeting
    Constraint::Length(1 + username_padding),                                                             // Username
    Constraint::Length(if greeter.mode == Mode::Username { message_height } else { 1 + answer_padding }), // Message or answer
    Constraint::Length(if greeter.mode == Mode::Password { message_height } else { 1 }),                  // Message
  ];

  let chunks = Layout::default().direction(Direction::Vertical).constraints(constraints.as_ref()).split(frame);
  let cursor = chunks[USERNAME_INDEX];

  if let Some(greeting) = &greeting {
    let greeting_text = Span::from(greeting.trim_end());
    let greeting_label = Paragraph::new(greeting_text).alignment(Alignment::Center);

    f.render_widget(greeting_label, chunks[GREETING_INDEX]);
  }

  let username_text = prompt_value(USERNAME);
  let username_label = Paragraph::new(username_text);

  let username_value_text = Span::from(greeter.username.clone());
  let username_value = Paragraph::new(username_value_text);

  match greeter.mode {
    Mode::Username | Mode::Password => {
      f.render_widget(username_label, chunks[USERNAME_INDEX]);
      f.render_widget(
        username_value,
        Rect::new(1 + chunks[USERNAME_INDEX].x + USERNAME.len() as u16, chunks[USERNAME_INDEX].y, get_input_width(greeter, USERNAME), 1),
      );

      let answer_text = if greeter.working { Span::from(WORKING) } else { prompt_value(&greeter.prompt) };
      let answer_label = Paragraph::new(answer_text);

      if greeter.mode == Mode::Password || greeter.previous_mode == Mode::Password {
        f.render_widget(answer_label, chunks[ANSWER_INDEX]);

        if !greeter.secret || greeter.asterisks {
          let value = if greeter.secret && greeter.asterisks {
            "*".repeat(greeter.answer.len())
          } else {
            greeter.answer.clone()
          };

          let answer_value_text = Span::from(value);
          let answer_value = Paragraph::new(answer_value_text);

          f.render_widget(
            answer_value,
            Rect::new(
              chunks[ANSWER_INDEX].x + greeter.prompt.chars().count() as u16,
              chunks[ANSWER_INDEX].y,
              get_input_width(greeter, &greeter.prompt),
              1,
            ),
          );
        }
      }

      if let Some(message) = message {
        let message_text = Span::from(message);
        let message = Paragraph::new(message_text);

        match greeter.mode {
          Mode::Username => f.render_widget(message, chunks[ANSWER_INDEX]),
          Mode::Password => f.render_widget(message, chunks[MESSAGE_INDEX]),
          Mode::Command | Mode::Sessions => {}
        }
      }
    }

    _ => {}
  }

  match greeter.mode {
    Mode::Username => {
      let offset = get_cursor_offset(greeter, greeter.username.chars().count());

      Ok((2 + cursor.x + USERNAME.len() as u16 + offset as u16, USERNAME_INDEX as u16 + cursor.y))
    }

    Mode::Password => {
      let offset = get_cursor_offset(greeter, greeter.answer.chars().count());

      if greeter.secret && !greeter.asterisks {
        Ok((1 + cursor.x + greeter.prompt.chars().count() as u16, ANSWER_INDEX as u16 + prompt_padding + cursor.y))
      } else {
        Ok((1 + cursor.x + greeter.prompt.chars().count() as u16 + offset as u16, ANSWER_INDEX as u16 + prompt_padding + cursor.y))
      }
    }

    _ => Ok((1, 1)),
  }
}
