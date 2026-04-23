use rand::{
  RngExt,
  rng,
};

/// Тип клетки
///
/// Base - ничего не делает
/// Trap - пропускает ход
/// Bonus - дополнительный ход
/// Teleport - телепортация на определённую клетку
enum CellType
{
  Base,
  Trap,
  Bonus,
  Teleport
  {
    destination: u32,
  },
}

struct Cell
{
  id: u32,
  cell_type: CellType,
}

/// Игрок
struct Player
{
  id: u32,
  pos: u32,
}

impl Player
{
  /// Игральный кубик у нас 6 максимум точек имеет
  fn random_walk() -> i32
  {
    let mut rng = rng();

    rng.random_range(1..=6)
  }
}

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

fn main()
{
  println!("Hello world!");
}
