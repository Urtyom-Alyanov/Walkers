using Xunit;
using Walkers.Core;

namespace Walkers.Core.Tests
{
    public class GameTests
    {
        [Fact]
        public void ShouldAddPlayers()
        {
            var game = new Game();
            game.AddPlayer("Player1");
            game.AddPlayer("Player2");
            Assert.Equal(2, game.Players.Count);
        }

        [Fact]
        public void ShouldRotateTurns()
        {
            var game = new Game();
            game.AddPlayer("P1");
            game.AddPlayer("P2");
            
            Assert.Equal("P1", game.CurrentPlayer.Name);
            game.NextTurn();
            Assert.Equal("P2", game.CurrentPlayer.Name);
            game.NextTurn();
            Assert.Equal("P1", game.CurrentPlayer.Name);
        }
    }
}
