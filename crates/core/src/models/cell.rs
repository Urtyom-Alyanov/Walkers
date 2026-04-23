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
}

pub struct Cell
{
  pub id: u32,
  pub cell_type: CellType,
}
