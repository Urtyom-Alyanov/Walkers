using System.Collections.Generic;

namespace Walkers.Core.Models;

public class Board
{
    public List<ITile> Tiles { get; } = new List<ITile>();
}
