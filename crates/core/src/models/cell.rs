use crate::models::{
  game::Game,
  player::{
    Player,
    PlayerState,
  },
};

/// Тип клетки
///
/// Base - ничего не делает
/// Trap - пропускает ход
/// Bonus - дополнительный ход
/// Teleport - телепортация на определённую клетку
pub enum CellType
{
  Base,
  Trap,
  Bonus,
  Teleport
  {
    destination: u32,
  },
  Finish,
}

pub struct Cell
{
  pub id: u32,
  pub cell_type: CellType,
}

impl Cell
{
  /// Позволяет в зависимости от типа клетки решать, куда ходит игрок, что ему присваивается, в каком он состоянии и тд.
  pub fn on_player_entering(&self, player: &mut Player, game: &mut Game)
  {
    let mut is_switch = true;

    match self.cell_type
    {
      CellType::Base =>
      {}
      CellType::Teleport { destination } =>
      {
        player.pos = destination;
      }
      CellType::Bonus =>
      {
        is_switch = false;
      }
      CellType::Trap =>
      {
        player.state = PlayerState::Block;
      }
      CellType::Finish =>
      {
        game.win(player);
      }
    }

    if is_switch
    {
      game.next_player();
    }
  }
}
