use std::{error::Error, io};

use termion::raw::RawTerminal;
use tui::{
  backend::TermionBackend,
  layout::Rect,
  style::{Modifier, Style},
  text::Span,
  widgets::{Block, BorderType, Borders, Paragraph},
  Frame,
};

use super::util::*;
use crate::Greeter;

const CHANGE_SESSION: &str = "Change session";

pub fn draw(greeter: &mut Greeter, f: &mut Frame<TermionBackend<RawTerminal<io::Stdout>>>) -> Result<(u16, u16), Box<dyn Error>> {
  let size = f.size();

  let width = greeter.width();
  let height: u16 = get_height(greeter) + greeter.sessions.len() as u16;
  let x = (size.width - width) / 2;
  let y = (size.height - height) / 2;

  let container = Rect::new(x, y, width, height);

  let title = Span::from(format!(" {} ", CHANGE_SESSION));
  let block = Block::default().title(title).borders(Borders::ALL).border_type(BorderType::Rounded);

  for (index, (name, _)) in greeter.sessions.iter().enumerate() {
    let name = format!("{:1$}", name, greeter.width() as usize - 4);

    let frame = Rect::new(x + 2, y + 2 + index as u16, width, 1);
    let option_text = get_option(&greeter, name, index);
    let option = Paragraph::new(option_text);

    f.render_widget(option, frame);
  }

  f.render_widget(block, container);

  Ok((1, 1))
}

fn get_option<'g, S>(greeter: &Greeter, name: S, index: usize) -> Span<'g>
where
  S: Into<String>,
{
  if greeter.selected_session == index {
    Span::styled(name.into(), Style::default().add_modifier(Modifier::REVERSED))
  } else {
    Span::from(name.into())
  }
}
