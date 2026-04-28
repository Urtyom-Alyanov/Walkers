use rand::{
  RngExt,
  rng,
};

use crate::models::{
  cell::{
    Cell,
    CellType,
  },
  player::{
    Player,
    PlayerState,
  },
};

/// Структура игры
pub struct Game
{
  pub players: Vec<Player>,
  pub cells: Vec<Cell>,
  pub current_player: u32,
  pub count_cells: usize,
}

pub struct GameOverStats {}

impl Game
{
  /// Начать игру с двумя игроками и 20-ю клетками
  pub fn start() -> Self
  {
    let count_players: usize = 2;
    let count_cells: usize = 40;

    let mut players: Vec<Player> = vec![];
    let mut cells: Vec<Cell> = vec![];

    for idx in 0..count_players
    {
      players.push(Player::new(idx.try_into().expect("Количество игроков слишком большое")));
    }

    let mut rng_handle = rng();

    for idx in 0..count_cells
    {
      let cell_type = if idx == count_cells - 1
      {
        CellType::Finish
      }
      else if idx == 0
      {
        CellType::Base
      }
      else
      {
        let type_rand: u32 = rng_handle.random_range(1..=10);
        if type_rand <= 2
        {
          CellType::Trap
        }
        else if type_rand <= 4
        {
          CellType::Bonus
        }
        else if type_rand == 5
        {
          let dest = rng_handle.random_range((idx + 1) as u32..count_cells as u32);
          CellType::Teleport { destination: dest }
        }
        else
        {
          CellType::Base
        }
      };

      cells.push(Cell { id: idx.try_into().expect("Количество клеток слишком большое"),
                        cell_type });
    }

    Self { players,
           cells,
           current_player: 0,
           count_cells }
  }

  /// Следующий игрок, если пересчёт окончен, то начинается новый круг.
  /// Начало нового круга можно проверить с помощью
  pub fn next_player(&mut self) -> u32
  {
    let current_player = self.current_player;

    for player in &mut self.players
    {
      if player.state == PlayerState::SemiBlock
      {
        player.state = PlayerState::Normal;
      }

      if player.state == PlayerState::Block
      {
        player.state = PlayerState::SemiBlock;
      }
    }

    for player in &self.players
    {
      if (player.id > current_player) && player.state == PlayerState::Normal
      {
        self.current_player = player.id;
        return self.current_player;
      }
    }

    self.current_player = 0;

    self.current_player
  }

  /// Завершает игру при победе определённого игрока
  pub fn win(&self) -> GameOverStats { GameOverStats {} }
}
