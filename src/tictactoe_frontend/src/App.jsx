import { useState } from 'react';
import Board from './components/Board/Board';
import GameStatus from './components/GameStatus/GameStatus';
import { calculateWinner, isDraw } from './utils/gameLogic';

function App() {
  const [board, setBoard] = useState(Array(9).fill(null));
  const [isXNext, setIsXNext] = useState(true);
  const winner = calculateWinner(board);

  const handleClick = (index) => {
    if (board[index] || winner) return;

    const newBoard = [...board];
    newBoard[index] = isXNext ? 'X' : 'O';
    setBoard(newBoard);
    setIsXNext(!isXNext);
  };

  const resetGame = () => {
    setBoard(Array(9).fill(null));
    setIsXNext(true);
  };

  return (
    <div className="game">
      <h1>Tic Tac Toe</h1>
      <GameStatus 
        winner={winner} 
        isXNext={isXNext} 
        isDraw={isDraw(board)} 
      />
      <Board 
        squares={board} 
        onSquareClick={handleClick} 
      />
      <button className="reset-button" onClick={resetGame}>
        Reset Game
      </button>
    </div>
  );
}

export default App;