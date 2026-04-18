using System;
using System.Windows.Forms;
using Walkers.Core;

namespace Walkers.FormsApp;

public partial class GameForm : Form
{
    private readonly Game _game;

    public GameForm()
    {
        InitializeComponent();

        _game = new Game();
        _game.AddPlayer("Игрок 1");
        _game.AddPlayer("Игрок 2");

        var btnRoll = new Button { Text = "Бросить кубик", Location = new System.Drawing.Point(10, 10), AutoSize = true };
        btnRoll.Click += BtnRoll_Click;
        Controls.Add(btnRoll);
    }

    private void BtnRoll_Click(object? sender, EventArgs e)
    {
        _game.NextTurn();
        MessageBox.Show($"Теперь ходит: {_game.CurrentPlayer.Name}");
    }
}

