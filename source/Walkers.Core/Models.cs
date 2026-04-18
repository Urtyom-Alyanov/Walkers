using System;
using System.Collections.Generic;

namespace Walkers.Core
{
    public interface ITile
    {
        int Id { get; }
        string Type { get; }
    }

    public class Tile : ITile
    {
        public int Id { get; }
        public string Type { get; }

        public Tile(int id, string type)
        {
            Id = id;
            Type = type;
        }
    }

    public class Player
    {
        public string Name { get; }
        public int Position { get; set; } = 0;

        public Player(string name)
        {
            Name = name;
        }
    }

    public class Board
    {
        public List<ITile> Tiles { get; } = new List<ITile>();
    }

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
}
