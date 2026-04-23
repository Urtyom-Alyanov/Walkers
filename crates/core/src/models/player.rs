use rand::{
  RngExt,
  rng,
};

/// Игрок
pub struct Player
{
  pub id: u32,
  pub pos: u32,
}

impl Player
{
  /// Игральный кубик у нас 6 максимум точек имеет
  pub fn random_walk() -> i32
  {
    let mut rng = rng();

    rng.random_range(1..=6)
  }
}
