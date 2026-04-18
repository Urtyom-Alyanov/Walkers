using System;
using System.Windows.Forms;

namespace Walkers.FormsApp;

public partial class MenuForm : Form
{
    public MenuForm()
    {
        Text = "Меню игры Walkers";
        Size = new System.Drawing.Size(300, 200);
        StartPosition = FormStartPosition.CenterScreen;

        var btnStart = new Button { Text = "Начать игру (Легко)", Location = new System.Drawing.Point(50, 50), AutoSize = true };
        btnStart.Click += (s, e) => {
            var gameForm = new GameForm();
            gameForm.Show();
            this.Hide();
        };
        Controls.Add(btnStart);
    }
}
