use crate::models::cell::{
  Cell,
  CellType,
};
use crate::models::player::{
  Player,
  PlayerState,
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
  fn start() -> Self
  {
    let count_players: usize = 2;
    let count_cells: usize = 20;

    let mut players: Vec<Player> = vec![];
    let mut cells: Vec<Cell> = vec![];

    for idx in 0..count_players
    {
      players.push(Player::new(idx.try_into().expect("Количество игроков слишком большое")));
    }

    for idx in 0..count_cells
    {
      cells.push(Cell { id: idx.try_into().expect("Количество клеток слишком большое"),
                        cell_type: CellType::Base });
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
    let current_player = self.current_player.clone();

    // Переходим к следующему не заблокированному игроку
    for player in &self.players
    {
      if (player.id > current_player) && player.state != PlayerState::Block
      {
        self.current_player = player.id.clone();
        break;
      }
    }

    // Если не сменилось, начинаем новый круг
    if current_player == self.current_player
    {
      self.current_player = 0;

      // Возвращаем "нормальный" статус заблокированным игрокам
      for player in &mut self.players
      {
        if player.state == PlayerState::Block
        {
          player.state = PlayerState::Normal;
          break;
        }
      }
    }

    // Возвращаем новый id-шник нового текущего игрока
    self.current_player
  }

  /// Завершает игру при победе определённого игрока
  pub fn win(&self, player: &Player) -> GameOverStats { GameOverStats {} }
}
