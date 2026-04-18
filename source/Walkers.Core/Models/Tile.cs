namespace Walkers.Core.Models;

public interface ITile
{
    int Id { get; }
    string Type { get; }
    int? TeleportTarget { get; }
}

public class Tile : ITile
{
    public int Id { get; }
    public string Type { get; }
    public int? TeleportTarget { get; }

    public Tile(int id, string type, int? teleportTarget = null)
    {
        Id = id;
        Type = type;
        TeleportTarget = teleportTarget;
    }
}
