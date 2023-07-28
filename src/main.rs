use iced::widget::{button, Column, Row, text};
use iced::{alignment::Horizontal, Alignment, Element, Sandbox, Settings};

const CELL_ROWS: usize = 16;
const CELL_COLUMNS: usize = 30;
const MINE_COUNT: usize = 99;

const SURROUNDING_CELLS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
];

pub fn main() -> iced::Result {
  let settings = Settings {
    window: iced::window::Settings { size: (20 * CELL_COLUMNS as u32, 30 + 20 * CELL_ROWS as u32), ..Default::default() },
    ..Default::default()
  };
  Game::run(settings)
}

#[derive(Clone, Copy, PartialEq)]
enum CellValue {
  Mined,
  Number(u8),
}

#[derive(PartialEq)]
enum GameStatus {
  Playing,
  Lost,
  Won,
}

#[derive(Clone, Copy)]
struct Cell {
  revealed: bool,
  value: CellValue,
}

struct Game {
  board: [[Cell; CELL_ROWS]; CELL_COLUMNS],
  status: GameStatus,
  revealed_count: usize
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
        for offset in &SURROUNDING_CELLS {
          let new_x = (x as isize + offset.0) as usize;
          let new_y = (y as isize + offset.1) as usize;
          //By converting it to usize, if the number equals -1, it will wrap around to be a really high number, which will not be counted below.
          
          if new_x < CELL_COLUMNS && new_y < CELL_ROWS
          && self.board[new_x][new_y].value == CellValue::Mined {
            count += 1;
          }
        }
        self.board[x][y].value = CellValue::Number(count);
      }
    }
  }
  
  fn reveal_multiple(&mut self, x: usize, y: usize) {
    let mut reveal_vec = vec![(x, y)];
    
    while let Some(cell) = reveal_vec.pop() {
      let x = cell.0;
      let y = cell.1;
    
      self.board[x][y].revealed = true;

      self.revealed_count += 1;
      if self.revealed_count >= CELL_ROWS * CELL_COLUMNS - MINE_COUNT {
        //All numbers were revealed
        self.status = GameStatus::Won;
        return;
      }
      
      //Clicked on a blank piece? Reveal all sides and corners.
      if self.board[x][y].value == CellValue::Number(0) {
        for offset in SURROUNDING_CELLS.iter() {
          let new_x = (x as isize + offset.0) as usize;
          let new_y = (y as isize + offset.1) as usize;
          //By converting it to usize, if the number equals -1, it will wrap around to be a really high number, which will not be counted below.

          if new_x < CELL_COLUMNS && new_y < CELL_ROWS && !self.board[new_x][new_y].revealed {
            reveal_vec.push((new_x, new_y));
          }
        }
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Message {
  NewGame,
  Position(usize, usize)
}

impl Sandbox for Game {
  type Message = Message;

  fn new() -> Self {
    let mut game = Game {
      board: [[Cell {revealed: false, value: CellValue::Number(0)}; CELL_ROWS]; CELL_COLUMNS],
      status: GameStatus::Playing,
      revealed_count: 0,
    };
    game.add_mines();
    game.add_numbers();
    
    game
  }

  fn title(&self) -> String {
    match self.status {
      GameStatus::Playing => String::from("Minesweeper"),
      GameStatus::Won => String::from("Minesweeper - You Won"),
      GameStatus::Lost => String::from("Minesweeper - You Lost"),
    }
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::NewGame => {
        *self = Game::new()
      },
      Message::Position(x, y) => {
        if self.status != GameStatus::Playing {
          return;
        }
    
        if self.board[x][y].value == CellValue::Mined {
          self.board[x][y].revealed = true;
          self.status = GameStatus::Lost;
          return;
        }
        
        self.reveal_multiple(x, y);
      }
    }
  }

  fn view(&self) -> Element<Message> {
    let mut column = Column::new();
    column = column.push(button("New Game").height(30).on_press(Message::NewGame)).align_items(Alignment::Center);
    for y in 0..CELL_ROWS {
      let mut row = Row::new();
      for x in 0..CELL_COLUMNS {
        let cell: Element<_> = match self.board[x][y] {
          Cell {revealed: false, .. } => {
            if self.status == GameStatus::Playing {
              button("").width(20).height(20).on_press(Message::Position(x, y)).into()                
            } else if self.status == GameStatus::Lost && self.board[x][y].value == CellValue::Mined {
              button(text("*").size(34).width(20).height(20).horizontal_alignment(Horizontal::Center)).width(20).height(20).padding(0).into()
            } else {
              button("").width(20).height(20).into()  //Removing on_press disables the buttons
            }
          },
          Cell {revealed: true, value: CellValue::Mined} => text("*").size(34).width(20).height(20).horizontal_alignment(Horizontal::Center).into(),
          Cell {revealed: true, value: CellValue::Number(0)} => text("").width(20).height(20).into(),
          Cell {revealed: true, value: CellValue::Number(number)} => text(number.to_string()).width(20).height(20).horizontal_alignment(Horizontal::Center).into(),
        };
        row = row.push(cell);
      }
      column = column.push(row);
    }
    column.into()
  }
}