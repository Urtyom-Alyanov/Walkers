namespace Walkers.Core.Models;

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
