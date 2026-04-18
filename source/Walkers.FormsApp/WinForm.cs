using System;
using System.Windows.Forms;

namespace Walkers.FormsApp;

public partial class WinForm : Form
{
    public WinForm(string winnerName)
    {
        Text = "Победа!";
        Size = new System.Drawing.Size(300, 150);
        StartPosition = FormStartPosition.CenterScreen;

        var lblWinner = new Label { Text = $"Победил: {winnerName}!", Dock = DockStyle.Fill, TextAlign = System.Drawing.ContentAlignment.MiddleCenter, Font = new System.Drawing.Font("Arial", 14, System.Drawing.FontStyle.Bold) };
        Controls.Add(lblWinner);
    }
}
