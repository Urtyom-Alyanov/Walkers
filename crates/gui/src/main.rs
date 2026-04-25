use core::models::{
  cell::CellType,
  game::Game,
  player::PlayerState,
};

use eframe::egui;

struct WalkersApp
{
  game: Option<Game>,
  winner: Option<u32>,
  dice_result: Option<u32>,
  game_started: bool,
}

impl Default for WalkersApp
{
  fn default() -> Self
  {
    Self { game: None,
           winner: None,
           dice_result: None,
           game_started: false }
  }
}

impl WalkersApp
{
  fn start_game(&mut self)
  {
    self.game = Some(Game::start());
    self.winner = None;
    self.dice_result = None;
    self.game_started = true;
  }

  fn roll_dice(&mut self)
  {
    if let Some(ref mut game) = self.game
    {
      let current_id = game.current_player;
      let count_cells = game.count_cells;

      let prev_pos = game.players[current_id as usize].pos;

      let new_pos = {
        let player = &mut game.players[current_id as usize];
        player.random_walk(game.count_cells)
      };

      let is_switch = {
        let mut is_switch = true;
        let player = &mut game.players[current_id as usize];
        if let Some(cell) = game.cells.get(new_pos as usize)
        {
          match cell.cell_type
          {
            CellType::Teleport { destination } =>
            {
              player.pos = destination;
              player.target_pos = destination;
            }
            CellType::Trap =>
            {
              player.state = PlayerState::Block;
            }
            CellType::Finish =>
            {
              self.winner = Some(current_id);
              return;
            }
            CellType::Bonus =>
            {
              is_switch = false;
            }
            _ =>
            {}
          }
        }

        is_switch
      };

      if is_switch
      {
        game.next_player();
      }

      let dice_res = new_pos - prev_pos;
      self.dice_result = Some(dice_res);

      if new_pos as usize >= count_cells - 1
      {
        self.winner = Some(current_id);
      }
    }
  }

  fn render_board(&self, ui: &mut egui::Ui)
  {
    let game = match &self.game
    {
      Some(g) => g,
      None => return,
    };

    let board_cols = 10;
    let cell_size = 60.;
    let spacing = 10.;

    let board_width = board_cols as f32 * (cell_size + spacing);
    let board_height =
      ((game.cells.len() + board_cols - 1) / board_cols) as f32 * (cell_size + spacing);

    let (response, painter) = ui.allocate_painter(egui::vec2(board_width + 50.,
                                                             board_height + 50.),
                                                  egui::Sense::hover());

    let base_pos = response.rect.min + egui::vec2(10., 10.);

    for grid_idx in 0..game.cells.len()
    {
      let row = grid_idx / board_cols;
      let col = grid_idx % board_cols;
      let is_reversed = row % 2 == 1;

      let display_idx = if is_reversed
      {
        row * board_cols + (board_cols - 1 - col)
      }
      else
      {
        grid_idx
      };

      if display_idx >= game.cells.len()
      {
        continue;
      }

      let cell = &game.cells[display_idx];

      let cell_color = match &cell.cell_type
      {
        CellType::Base => egui::Color32::from_rgb(200, 200, 200),
        CellType::Trap => egui::Color32::from_rgb(255, 100, 100),
        CellType::Bonus => egui::Color32::from_rgb(100, 255, 100),
        CellType::Teleport { .. } => egui::Color32::from_rgb(100, 100, 255),
        CellType::Finish => egui::Color32::from_rgb(255, 215, 0),
      };

      let cell_label_type: &str = match &cell.cell_type
      {
        CellType::Base => "🏠",
        CellType::Trap => "🪤",
        CellType::Bonus => "🎁",
        CellType::Teleport { destination } => &format!("🔀 -> {}", destination),
        CellType::Finish => "🏁",
      };

      let cell_label = format!("{} {}", display_idx, cell_label_type);

      let x = base_pos.x + col as f32 * (cell_size + spacing);
      let y = base_pos.y + row as f32 * (cell_size + spacing);

      let rect = egui::Rect::from_min_size(egui::Pos2::new(x, y), egui::vec2(cell_size, cell_size));

      painter.rect_filled(rect, 2., cell_color);

      let is_occupied = game.players.iter().any(|p| p.target_pos == cell.id as u32);

      let button_text = if is_occupied
      {
        cell_label.to_string()
      }
      else
      {
        cell_label.to_string()
      };

      let text_pos = egui::Pos2::new(x + cell_size / 2. - 10., y + cell_size / 2. - 10.);
      painter.text(text_pos,
                   egui::Align2::CENTER_CENTER,
                   button_text,
                   egui::FontId::new(14., egui::FontFamily::Proportional),
                   egui::Color32::BLACK);

      let arrow_color = egui::Color32::from_rgb(50, 150, 50);

      if is_reversed
      {
        if col > 0
        {
          let arrow_rect = egui::Rect::from_min_size(egui::Pos2::new(x - spacing / 2. - 5.,
                                                                     y + cell_size / 2. - 3.),
                                                     egui::vec2(spacing, 6.));
          painter.rect_filled(arrow_rect, 0., arrow_color);
        }
      }
      else
      {
        if col < board_cols - 1 && grid_idx + 1 < game.cells.len()
        {
          let arrow_rect = egui::Rect::from_min_size(egui::Pos2::new(x + cell_size + spacing / 2.
                                                                     - 5.,
                                                                     y + cell_size / 2. - 3.),
                                                     egui::vec2(spacing, 6.));
          painter.rect_filled(arrow_rect, 0., arrow_color);
        }
      }

      if grid_idx + board_cols < game.cells.len()
         && ((is_reversed && col == 0) || (!is_reversed && col == board_cols - 1))
      {
        let arrow_rect = egui::Rect::from_min_size(egui::Pos2::new(x + cell_size / 2. - 3.,
                                                                   y + cell_size + spacing / 2.
                                                                   - 5.),
                                                   egui::vec2(6., spacing));
        painter.rect_filled(arrow_rect, 0., arrow_color);
      }
    }

    for (i, player) in game.players.iter().enumerate()
    {
      let target_grid_idx = player.target_pos as usize;
      let target_row = target_grid_idx / board_cols;
      let target_col = target_grid_idx % board_cols;
      let target_is_reversed = target_row % 2 == 1;

      let target_display_col = if target_is_reversed
      {
        board_cols - 1 - target_col
      }
      else
      {
        target_col
      };

      let target_x = base_pos.x + target_display_col as f32 * (cell_size + spacing);
      let target_y = base_pos.y + target_row as f32 * (cell_size + spacing);

      let offset_x = if i == 1 { cell_size / 4. } else { 0. };
      let offset_y = if i == 1 { cell_size / 4. } else { 0. };

      let player_rect =
        egui::Rect::from_min_size(egui::Pos2::new(target_x + cell_size / 4. + offset_x,
                                                  target_y + cell_size / 4. + offset_y),
                                  egui::vec2(cell_size / 2., cell_size / 2.));

      let player_color = if player.id == 0
      {
        egui::Color32::from_rgb(0, 150, 255)
      }
      else
      {
        egui::Color32::from_rgb(255, 100, 0)
      };

      painter.rect_filled(player_rect, 4., player_color);
      painter.text(egui::Pos2::new(target_x + cell_size / 2. + offset_x,
                                   target_y + cell_size / 2. + offset_y),
                   egui::Align2::CENTER_CENTER,
                   format!("P{}", player.id),
                   egui::FontId::new(12., egui::FontFamily::Proportional),
                   egui::Color32::WHITE);
    }
  }

  fn render_players(&self, ui: &mut egui::Ui)
  {
    let game = match &self.game
    {
      Some(g) => g,
      None => return,
    };

    ui.separator();
    ui.label(egui::RichText::new("Игроки").heading());

    egui::Grid::new("players").num_columns(2)
                              .spacing([10., 5.])
                              .show(ui, |ui| {
                                for player in &game.players
                                {
                                  let player_name = format!("Игрок {}", player.id);
                                  let player_color = if player.id == game.current_player
                                  {
                                    egui::Color32::from_rgb(0, 200, 0)
                                  }
                                  else
                                  {
                                    egui::Color32::GRAY
                                  };

                                  let status = match player.state
                                  {
                                    PlayerState::Normal => "Активен",
                                    PlayerState::Block => "Заблокирован",
                                    PlayerState::SemiBlock => "Заблокирован (СКОРО)",
                                  };

                                  ui.label(egui::RichText::new(player_name).color(player_color));
                                  ui.label(format!("Поз: {} | {}", player.pos, status));
                                  ui.end_row();
                                }
                              });
  }
}

impl eframe::App for WalkersApp
{
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
  {
    if let Some(ref mut game) = self.game
    {
      for player in &mut game.players
      {
        player.update_animation(0.5);
      }
    }

    egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Бродилка - Настольная игра");

            ui.separator();

            if !self.game_started
            {
                if ui.button("Начать игру").clicked()
                {
                    self.start_game();
                }
            }
            else
            {
                ui.vertical(|ui| {
                    self.render_board(ui);
                    self.render_players(ui);

                    ui.separator();

                    if let Some(winner) = self.winner
                    {
                        ui.label(egui::RichText::new(format!("Победитель: Игрок {}", winner)).heading().color(egui::Color32::GOLD));
                        if ui.button("Играть снова").clicked()
                        {
                            self.start_game();
                        }
                    }
                    else
                    {
                        if let Some(result) = self.dice_result
                        {
                            ui.label(format!("Выпало: {}", result));
                        }

                        if ui.button("Бросить кубик").clicked()
                        {
                            self.roll_dice();
                        }
                    }
                });
            }
        });
  }
}

fn main() -> eframe::Result<()>
{
  let options = eframe::NativeOptions::default();

  eframe::run_native("Бродилка",
                     options,
                     Box::new(|_cc| Ok(Box::new(WalkersApp::default()))))
}
