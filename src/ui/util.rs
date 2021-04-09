use crate::{Greeter, Mode};

pub fn get_height(greeter: &Greeter) -> u16 {
  let container_padding = greeter.container_padding();
  let prompt_padding = greeter.prompt_padding();
  let (_, message_height) = get_message_height(greeter, 2, 0);
  let (_, greeting_height) = get_greeting_height(greeter, 1, 0);

  let initial = match greeter.mode {
    Mode::Username | Mode::Command => (2 * container_padding) + 1,
    Mode::Password => (2 * container_padding) + prompt_padding + 1,
    Mode::Sessions => (2 * container_padding),
  };

  match greeter.mode {
    Mode::Command | Mode::Sessions => initial,
    _ => initial + greeting_height + message_height,
  }
}

pub fn get_input_width(greeter: &Greeter, label: &str) -> u16 {
  greeter.width() - label.chars().count() as u16 - 4 - 1
}

pub fn get_cursor_offset(greeter: &mut Greeter, length: usize) -> i16 {
  let mut offset = length as i16 + greeter.cursor_offset;

  if offset < 0 {
    offset = 0;
    greeter.cursor_offset = -(length as i16);
  }

  if offset > length as i16 {
    offset = length as i16;
    greeter.cursor_offset = 0;
  }

  offset
}

pub fn get_greeting_height(greeter: &Greeter, padding: u16, fallback: u16) -> (Option<String>, u16) {
  if let Some(greeting) = &greeter.greeting {
    let width = greeter.width();
    let wrapped = textwrap::fill(greeting, width as usize - 4);
    let height = wrapped.trim_end().matches('\n').count();

    (Some(wrapped), height as u16 + 1 + padding)
  } else {
    (None, fallback)
  }
}

pub fn get_message_height(greeter: &Greeter, padding: u16, fallback: u16) -> (Option<String>, u16) {
  if let Some(message) = &greeter.message {
    let width = greeter.width();
    let wrapped = textwrap::fill(message.trim_end(), width as usize - 4);
    let height = wrapped.trim_end().matches('\n').count();

    (Some(wrapped), height as u16 + padding)
  } else {
    (None, fallback)
  }
}
