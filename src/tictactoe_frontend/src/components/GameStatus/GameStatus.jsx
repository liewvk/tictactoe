function GameStatus({ winner, isXNext, isDraw }) {
    const status = winner
      ? `Winner: ${winner}`
      : isDraw
      ? "It's a draw!"
      : `Next player: ${isXNext ? 'X' : 'O'}`;
  
    return <div className="status">{status}</div>;
  }
  
  export default GameStatus;