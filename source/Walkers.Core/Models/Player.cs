namespace Walkers.Core.Models;

public class Player
{
    public string Name { get; }
    public int Position { get; set; } = 0;

    public Player(string name)
    {
        Name = name;
    }
}
