using System;
using System.Drawing;
using System.Windows.Forms;
using Walkers.Core;
using Walkers.Core.Models;
using System.Linq;

namespace Walkers.FormsApp;

public partial class GameForm : Form
{
    private readonly Game _game;
    private const int TileSize = 50;
    private ListBox _gameLog;

    public GameForm()
    {
        InitializeComponent();

        _game = new Game();
        _game.AddPlayer("Игрок 1");
        _game.AddPlayer("Игрок 2");
        
        for (int i = 0; i < 50; i++)
        {
            string type = "Normal";
            int? teleport = null;
            if (i == 0) type = "Start";
            else if (i == 49) type = "Finish";
            else if (i == 10) { type = "Teleport"; teleport = 30; }
            else if (i == 40) { type = "Teleport"; teleport = 20; }
            else if (i % 7 == 0) type = "Trap";
            else if (i % 9 == 0) type = "Bonus";
            
            _game.Board.Tiles.Add(new Tile(i, type, teleport));
        }

        var btnRoll = new Button { Text = "Бросить кубик", Location = new Point(600, 50), AutoSize = true };
        btnRoll.Click += BtnRoll_Click;
        Controls.Add(btnRoll);

        _gameLog = new ListBox { Location = new Point(600, 100), Size = new Size(200, 300) };
        Controls.Add(_gameLog);
        
        var lblTurn = new Label { Text = $"Ход: {_game.CurrentPlayer.Name}", Location = new Point(600, 20), AutoSize = true, Font = new Font(Font, FontStyle.Bold) };
        Controls.Add(lblTurn);
        
        this.Paint += GameForm_Paint;
    }

    private void AddLog(string message, string status = "") 
    {
        _gameLog.Items.Add($"{DateTime.Now:HH:mm:ss} - {message}");
        
        var lblTurn = Controls.OfType<Label>().FirstOrDefault();
        if (lblTurn != null)
        {
            lblTurn.Text = $"Ход: {_game.CurrentPlayer.Name} {status}";
            lblTurn.ForeColor = _game.CurrentPlayer.Name == "Игрок 1" ? Color.Goldenrod : Color.Blue;
        }
    }

    private void BtnRoll_Click(object? sender, EventArgs e)
    {
        Random rnd = new Random();
        int dice = rnd.Next(1, 7);
        var p = _game.CurrentPlayer;
        int newPos = Math.Min(p.Position + dice, 49);
        bool extraTurn = false;
        string status = "";

        var tile = _game.Board.Tiles[newPos];
        if (tile.TeleportTarget.HasValue)
        {
            newPos = tile.TeleportTarget.Value;
            AddLog($"{p.Name} на портале -> {newPos}");
        }
        else if (tile.Type == "Trap")
        {
            status = "(Ловушка!)";
            AddLog($"{p.Name} в ловушке, пропуск хода!");
            _game.NextTurn(); // Пропуск текущего хода при следующем клике
        }
        else if (tile.Type == "Bonus")
        {
            extraTurn = true;
            status = "(Бонус!)";
            AddLog($"{p.Name} получил бонус!");
        }

        p.Position = newPos;

        if (p.Position == 49)
        {
            AddLog($"{p.Name} победил!");
            new WinForm(p.Name).ShowDialog();
            this.Close();
            return;
        }

        if (!extraTurn) _game.NextTurn();

        this.Invalidate();
        AddLog($"{p.Name} выбросил {dice}. Позиция: {p.Position}", status);

        var lblTurn = Controls.OfType<Label>().FirstOrDefault();
        if (lblTurn != null) lblTurn.Text = $"Ход: {_game.CurrentPlayer.Name}";
    }

    private void GameForm_Paint(object? sender, PaintEventArgs e)
    {
        Graphics g = e.Graphics;
        for (int i = 0; i < _game.Board.Tiles.Count; i++)
        {
            int row = i / 10;
            bool reverseRow = row % 2 != 0;
            int col = reverseRow ? (9 - (i % 10)) : (i % 10);
            
            Rectangle rect = new Rectangle(10 + col * (TileSize + 5), 50 + row * (TileSize + 5), TileSize, TileSize);
            
            Brush tileBrush = _game.Board.Tiles[i].Type switch {
                "Start" => Brushes.Green,
                "Finish" => Brushes.Gold,
                "Teleport" => Brushes.Purple,
                "Trap" => Brushes.OrangeRed,
                "Bonus" => Brushes.LightBlue,
                _ => Brushes.White
            };
            
            g.FillRectangle(tileBrush, rect);
            g.DrawRectangle(Pens.Black, rect);
            
            string label = $"{i}";
            if (_game.Board.Tiles[i].TeleportTarget.HasValue) label += $"->{_game.Board.Tiles[i].TeleportTarget}";
            g.DrawString(label, this.Font, Brushes.Black, rect);
        }

        foreach (var player in _game.Players)
        {
            int row = player.Position / 10;
            bool reverseRow = row % 2 != 0;
            int col = reverseRow ? (9 - (player.Position % 10)) : (player.Position % 10);
            
            g.FillEllipse(player.Name == "Игрок 1" ? Brushes.Yellow : Brushes.Blue, 10 + col * (TileSize + 5) + 10, 50 + row * (TileSize + 5) + 10, 20, 20);
        }

        // Легенда
        int legY = 350;
        int x = 10;
        
        string[] legend = { "Start", "Finish", "Teleport", "Trap", "Bonus" };
        Brush[] brushes = { Brushes.Green, Brushes.Gold, Brushes.Purple, Brushes.OrangeRed, Brushes.LightBlue };
        
        for (int i = 0; i < legend.Length; i++)
        {
            g.DrawString(legend[i], this.Font, brushes[i], x, legY);
            x += (int)g.MeasureString(legend[i], this.Font).Width + 10;
        }
    }
}
