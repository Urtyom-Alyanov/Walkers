use crate::models::cell::{
  Cell,
  CellType,
};
use crate::models::player::Player;

/// Структура игры
struct Game
{
  players: Vec<Player>,
  cells: Vec<Cell>,
}

impl Game
{
  /// Начать игру с двумя игроками и 20-ю клетками
  fn start_game() -> Self
  {
    let count_players: usize = 2;
    let count_cells: usize = 20;

    let mut players: Vec<Player> = vec![];
    let mut cells: Vec<Cell> = vec![];

    for idx in 0..count_players
    {
      players.push(Player { id: idx.try_into().expect("Количество игроков слишком большое"),
                            pos: 0 });
    }

    for idx in 0..count_cells
    {
      cells.push(Cell { id: idx.try_into().expect("Количество клеток слишком большое"),
                        cell_type: CellType::Base });
    }

    Self { players, cells }
  }
}
