use iced::widget::{button, Column, Row, text};
use iced::{alignment::Horizontal, Element, Sandbox, Settings};

const CELL_COLUMNS: usize = 16;
const CELL_ROWS: usize = 16;
const MINE_COUNT: usize = 40;

pub fn main() -> iced::Result {
  let settings = Settings {
    window: iced::window::Settings { size: (20 * CELL_ROWS as u32, 20 * CELL_COLUMNS as u32), ..Default::default() },
    ..Default::default()
  };
  Minesweeper::run(settings)
}

#[derive(Debug, Clone, Copy)]
struct Cell {
  revealed: bool,
  mined: bool,
  number: u8,
}

struct Minesweeper {
  board: [[Cell; CELL_COLUMNS]; CELL_COLUMNS],
}

impl Minesweeper {
  fn add_mines(&mut self) {
    use rand::seq::SliceRandom;
    
    let mut rng = rand::thread_rng();
    
    // Create a Vec of all possible positions.
    let mut positions = Vec::new();
    for y in 0..CELL_COLUMNS {
      for x in 0..CELL_ROWS {
        positions.push((x, y));
      }
    }
    
    // Shuffle the Vec of positions.
    positions.shuffle(&mut rng);
    
    // Mine some positions.
    for &(x, y) in positions.iter().take(MINE_COUNT) {
      self.board[x][y].mined = true;
    }
  }

  fn add_numbers(&mut self) {
    for y in 0..CELL_COLUMNS {
      let first_y = y == 0;
      let last_y = y == CELL_COLUMNS - 1;
      
      for x in 0..CELL_ROWS {
        let first_x = x == 0;
        let last_x = x == CELL_ROWS - 1;
        
        if self.board[x][y].mined {
          continue;
        }
        
        //Count up all bombs at sides and corners
        let mut count = 0;
        if !first_x && !first_y && self.board[x - 1][y - 1].mined { count += 1 }
        if !first_x && self.board[x - 1][y].mined { count += 1 }
        if !first_y && self.board[x][y - 1].mined { count += 1 }
        if !last_x && !last_y && self.board[x + 1][y + 1].mined { count += 1 }
        if !last_x && self.board[x + 1][y].mined { count += 1 }
        if !last_y && self.board[x][y + 1].mined { count += 1 }
        if !first_x && !last_y && self.board[x - 1][y + 1].mined { count += 1 }
        if !last_x && !first_y && self.board[x + 1][y - 1].mined { count += 1 }
        
        self.board[x][y].number = count;
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Message {
  Position(usize, usize)
}

impl Sandbox for Minesweeper {
  type Message = Message;

  fn new() -> Self {
    let mut game = Minesweeper { board: [[Cell {revealed: false, mined: false, number: 0}; CELL_ROWS]; CELL_COLUMNS] };
    game.add_mines();
    game.add_numbers();
    
    game
  }

  fn title(&self) -> String {
    String::from("Minesweeper")
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::Position(x, y) => {
        //This is already revealed. No need to do anything here.
        if self.board[x][y].revealed {
          return;
        }
        
        self.board[x][y].revealed = true;
        if self.board[x][y].mined {
          return;
        }
        
        //Clicked on a blank piece? Reveal all sides and corners.
        if self.board[x][y].number == 0 {
          let first_y = y == 0;
          let last_y = y == CELL_COLUMNS - 1;
          let first_x = x == 0;
          let last_x = x == CELL_ROWS - 1;
          
          if !first_x && !first_y { self.update(Message::Position(x - 1, y - 1)); }
          if !first_x { self.update(Message::Position(x - 1, y)); }
          if !first_y { self.update(Message::Position(x, y - 1)); }
          if !last_x && !last_y { self.update(Message::Position(x + 1, y + 1)); }
          if !last_x { self.update(Message::Position(x + 1, y)); }
          if !last_y { self.update(Message::Position(x, y + 1)); }
          if !first_x && !last_y { self.update(Message::Position(x - 1, y + 1)); }
          if !last_x && !first_y { self.update(Message::Position(x + 1, y - 1)); }
        }
      }
    }
  }

  fn view(&self) -> Element<Message> {
    let mut column = Column::new();
    for y in 0..CELL_COLUMNS {
      let mut row = Row::new();
      for x in 0..CELL_ROWS {
        let cell: Element<_> = match self.board[x][y] {
          Cell {revealed: false, .. } => button("").width(20).height(20).on_press(Message::Position(x, y)).into(),
          Cell {revealed: true, mined: true, .. } => text("*").width(20).height(20).horizontal_alignment(Horizontal::Center).into(),
          Cell {revealed: true, mined: false, number: 0 } => text("").width(20).height(20).into(),
          Cell {revealed: true, mined: false, number } => text(number.to_string()).width(20).height(20).horizontal_alignment(Horizontal::Center).into(),
        };
        row = row.push(cell);
      }
      column = column.push(row);
    }
    column.into()
  }
}