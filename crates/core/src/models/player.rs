use rand::{
  RngExt,
  rng,
};

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerState
{
  Normal,
  Block,
  SemiBlock,
}

pub struct Player
{
  pub id: u32,
  pub pos: u32,
  pub anim_pos: f32,
  pub target_pos: u32,
  pub state: PlayerState,
}

impl Player
{
  pub fn new(id: u32) -> Self
  {
    Self { pos: 0,
           anim_pos: 0.,
           target_pos: 0,
           id,
           state: PlayerState::Normal }
  }

  pub fn random_walk(&mut self, count_cells: usize) -> u32
  {
    let count_cells_as_u32: u32 =
      count_cells.try_into()
                 .expect("Непредвиденная ошибка(`count_cells` не вмещается в тип u32)");

    let mut rng = rng();
    let count_walk: u32 = rng.random_range(1..=6);

    let positions_ost: u32 = count_cells_as_u32 - self.pos - 1;

    let count_walk = if positions_ost < count_walk
    {
      positions_ost
    }
    else
    {
      count_walk
    };

    let new_pos = self.pos + count_walk;

    self. target_pos= new_pos;
    self.pos = new_pos;

    new_pos
  }

  pub fn update_animation(&mut self, speed: f32)
  {
    if (self.anim_pos as u32) != self.target_pos
    {
      let direction = if self.target_pos > self.anim_pos as u32
      {
        1.
      }
      else
      {
        -1.
      };

      self.anim_pos += speed * direction;

      if (self.anim_pos as u32) == self.target_pos
      {
        self.anim_pos = self.target_pos as f32;
      }
    }
  }
}
