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
