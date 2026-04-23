use rand::{
  RngExt,
  rng,
};

use crate::models::game::Game;

#[derive(PartialEq)]
pub enum PlayerState
{
  Normal,
  Block,
}

/// Структура игрока
pub struct Player
{
  pub id: u32,
  pub pos: u32,
  pub state: PlayerState,
}

impl Player
{
  pub fn new(id: u32) -> Self
  {
    Self { pos: 0,
           id,
           state: PlayerState::Normal }
  }

  /// Игральный кубик у нас 6 максимум точек имеет
  ///
  /// Меняет пользователя и возвращает новую позицию
  pub fn random_walk(&mut self, game: &Game) -> u32
  {
    let count_cells_as_u32: u32 =
      game.count_cells
          .try_into()
          .expect("Непредвиденная ошибка( `count_cells` не вмещается в тип u32");

    let mut rng = rng();
    let mut count_walk: u32 = rng.random_range(1..=6);

    let positions_ost: u32 = count_cells_as_u32 - self.pos;

    if positions_ost < count_walk
    {
      count_walk = positions_ost;
    }

    let new_pos = self.pos + count_walk;

    self.pos = new_pos;

    new_pos
  }
}
