#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cell;
use cell::CellWidget;

use iced::widget::{Column, Row, Space};
use iced::{alignment, Element, Sandbox, Settings};

const CELL_ROWS: usize = 16;
const CELL_COLUMNS: usize = 30;
const MINE_COUNT: usize = 99;

pub fn main() -> iced::Result {
  let settings = Settings {
    window: iced::window::Settings {
      size: (21 * CELL_COLUMNS as u32, 33 + 21 * CELL_ROWS as u32),
      resizable: false,
      ..Default::default()
    },
    ..Default::default()
  };
  Game::run(settings)
}

#[derive(Clone, Copy, PartialEq)]
enum CellValue {
  Mined,
  Number(u8),
}

#[derive(Clone, Copy, PartialEq)]
enum CellStatus {
  Covered,
  Revealed,
  Flagged,
}

#[derive(Clone, Copy)]
struct Cell {
  status: CellStatus,
  value: CellValue,
}

#[derive(PartialEq)]
enum GameStatus {
  Playing,
  Pressing,
  Lost,
  Won,
}

struct Game {
  board: [[Cell; CELL_ROWS]; CELL_COLUMNS],
  status: GameStatus,
  revealed_count: usize,
  flag_count: usize,
}

fn with_surrounding_cells<F>(x: usize, y: usize, mut f: F) where F: FnMut(usize, usize) {
  let first_y = y == 0;
  let last_y = y == CELL_ROWS - 1;
  let first_x = x == 0;
  let last_x = x == CELL_COLUMNS - 1;
  
  if !first_x && !first_y { f(x - 1, y - 1) }
  if !first_x { f(x - 1, y) }
  if !first_y { f(x, y - 1) }
  if !last_x && !last_y { f(x + 1, y + 1) }
  if !last_x { f(x + 1, y) }
  if !last_y { f(x, y + 1) }
  if !first_x && !last_y { f(x - 1, y + 1) }
  if !last_x && !first_y { f(x + 1, y - 1) }
}

impl Game {
  fn add_mines(&mut self) {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    
    // Create a Vec of all possible positions.
    let mut positions = Vec::new();
    for y in 0..CELL_ROWS {
      for x in 0..CELL_COLUMNS {
        positions.push((x, y));
      }
    }
    
    // Shuffle the Vec of positions.
    positions.shuffle(&mut rng);
    
    // Mine some positions.
    for &(x, y) in positions.iter().take(MINE_COUNT) {
      self.board[x][y].value = CellValue::Mined;
    }
  }
  
  fn add_numbers(&mut self) {
    for y in 0..CELL_ROWS {
      for x in 0..CELL_COLUMNS {
        if self.board[x][y].value == CellValue::Mined {
          continue;
        }
        //Count up all bombs at sides and corners
        let mut count = 0;
        with_surrounding_cells(x, y, |new_x, new_y| {
          if self.board[new_x][new_y].value == CellValue::Mined {
            count += 1;
          }
        });
        self.board[x][y].value = CellValue::Number(count);
      }
    }
  }
  
  fn reveal_multiple(&mut self, x: usize, y: usize) {
    let mut reveal_vec = vec![(x, y)];
    
    while let Some(cell) = reveal_vec.pop() {
      let x = cell.0;
      let y = cell.1;

      //We can't be revealing flagged cells...
      if self.board[x][y].status == CellStatus::Flagged {
        continue;
      }

      self.board[x][y].status = CellStatus::Revealed;

      self.revealed_count += 1;
      if self.revealed_count >= CELL_ROWS * CELL_COLUMNS - MINE_COUNT {
        //All numbers were revealed
        self.status = GameStatus::Won;
        return;
      }
      
      //Clicked on a blank piece? Reveal all sides and corners.
      if self.board[x][y].value == CellValue::Number(0) {
        with_surrounding_cells(x, y, |new_x, new_y| {
          if self.board[new_x][new_y].status != CellStatus::Revealed {
            reveal_vec.push((new_x, new_y));
          }
        });
      }
    }
  }
}

fn text_color(number: u8) -> iced::Color {
  match number {
    1 => iced::Color::new(0.0, 0.0, 1.0, 0.0),  //Blue
    2 => iced::Color::new(0.0, 0.5, 0.0, 0.0),  //Green
    3 => iced::Color::new(1.0, 0.0, 0.0, 0.0),  //Red
    4 => iced::Color::new(0.0, 0.0, 0.5, 0.0),  //Dark blue
    5 => iced::Color::new(0.5, 0.0, 0.0, 0.0),  //Dark red
    6 => iced::Color::new(0.0, 0.5, 0.5, 0.0),  //Cyan
    7 => iced::Color::new(0.0, 0.0, 0.0, 0.0),  //Black
    8 => iced::Color::new(0.5, 0.5, 0.5, 0.0),  //Grey
    _ => iced::Color::new(1.0, 1.0, 1.0, 0.0),  //White
  }
}

fn text<'a>(text: impl Into<std::borrow::Cow<'a, str>>) -> iced::widget::Text<'a> {
  iced::widget::Text::new(text).shaping(iced::widget::text::Shaping::Advanced).size(16).width(20).height(20).horizontal_alignment(alignment::Horizontal::Center)
}

#[derive(Clone, Copy, Debug)]
enum Message {
  NewGame,
  Pressing(bool),
  Reveal(usize, usize),
  Flag(usize, usize),
}

impl Sandbox for Game {
  type Message = Message;

  fn new() -> Self {
    let mut game = Game {
      board: [[Cell {status: CellStatus::Covered, value: CellValue::Number(0)}; CELL_ROWS]; CELL_COLUMNS],
      status: GameStatus::Playing,
      revealed_count: 0,
      flag_count: 0,
    };
    game.add_mines();
    game.add_numbers();
    
    game
  }

  fn title(&self) -> String {
    match self.status {
      GameStatus::Won => String::from("Minesweeper - You Won"),
      GameStatus::Lost => String::from("Minesweeper - You Lost"),
      _ => String::from("Minesweeper"),
    }
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::NewGame => *self = Game::new(),
      Message::Pressing(true) => self.status = GameStatus::Pressing,
      Message::Pressing(false) => self.status = GameStatus::Playing,
      Message::Reveal(x, y) => {
        if self.status != GameStatus::Playing || self.board[x][y].status != CellStatus::Covered {
          return;
        }

        if self.board[x][y].value == CellValue::Mined {
          self.board[x][y].status = CellStatus::Revealed;
          self.status = GameStatus::Lost;
          return;
        }
        
        self.reveal_multiple(x, y);
      },
      Message::Flag(x, y) => {
        if self.status != GameStatus::Playing {
          return;
        }
        
        match self.board[x][y].status {
          CellStatus::Covered => {
            if MINE_COUNT == self.flag_count {
              //Too many flags! Don't add an extra flag. (Else MNE_COUNT - self.flag_count < 0, which will cause an exception because they are unsigned.)
              return;
            }
            self.board[x][y].status = CellStatus::Flagged;
            self.flag_count += 1;
          },
          CellStatus::Flagged => {
            self.board[x][y].status = CellStatus::Covered;
            self.flag_count -= 1;
          },
          CellStatus::Revealed => (), //If it's already revealed, it can't be flagged.
        };
        
      },
    }
  }

  fn view(&self) -> Element<Message> {
    let mut column = Column::new().spacing(1);
    let face = match self.status {
      GameStatus::Playing => "😀",
      GameStatus::Pressing => "😮",
      GameStatus::Lost => "☹️",
      GameStatus::Won => "😎",
    };
    let mut top_row = Row::new().padding(2);
    top_row = top_row.push(iced::widget::Text::new(format!("Mines: {}", MINE_COUNT - self.flag_count)).size(20));
    top_row = top_row.push(Space::with_width(iced::Length::Fill));
    top_row = top_row.push(iced::widget::Button::new(text(face).size(18)).height(28).on_press(Message::NewGame));
    top_row = top_row.push(Space::with_width(iced::Length::Fill));
    top_row = top_row.push(iced::widget::Text::new("No clock").size(20));
    column = column.push(top_row);
    for y in 0..CELL_ROWS {
      let mut row = Row::new().spacing(1);
      for x in 0..CELL_COLUMNS {
        let cell: Element<_> = match self.board[x][y] {
          Cell {status: CellStatus::Flagged, .. } => CellWidget {content: "🚩".to_string(), size: 14.0, padding: 2.into(), on_right_click: Some(Message::Flag(x, y)), ..Default::default()}.into(),
          Cell {status: CellStatus::Covered, .. } => match self.status {
            GameStatus::Playing | GameStatus::Pressing => {
              CellWidget {
                on_press: Some(Message::Pressing(true)),
                on_release: Some(Message::Pressing(false)),
                on_left_click: Some(Message::Reveal(x, y)),
                on_right_click: Some(Message::Flag(x, y)),
                ..Default::default()}.into()                
            },
            GameStatus::Won | GameStatus::Lost => if self.board[x][y].value == CellValue::Mined {
              CellWidget {content: "💣".to_string(), ..Default::default()}.into()
            } else {
              CellWidget {content: "".to_string(), ..Default::default()}.into()  //Removing on_press disables the buttons
            },
          },
          Cell {status: CellStatus::Revealed, value: CellValue::Mined} => text("💣").into(),
          Cell {status: CellStatus::Revealed, value: CellValue::Number(0)} => text("").into(),
          Cell {status: CellStatus::Revealed, value: CellValue::Number(number)} => text(number.to_string()).size(20).style(text_color(number)).font(iced::Font::MONOSPACE).into(),
        };
        row = row.push(cell);
      }
      column = column.push(row);
    }
    column.into()
  }
}