using System.Collections.Generic;
using Walkers.Core.Models;

namespace Walkers.Core;

public class Game
{
    public Board Board { get; } = new Board();
    public List<Player> Players { get; } = new List<Player>();
    public int CurrentPlayerIndex { get; private set; } = 0;

    public void AddPlayer(string name)
    {
        Players.Add(new Player(name));
    }

    public Player CurrentPlayer => Players[CurrentPlayerIndex];

    public void NextTurn()
    {
        CurrentPlayerIndex = (CurrentPlayerIndex + 1) % Players.Count;
    }
}
