import { useState } from 'react';

export default function FenInput({ onLoadFen }) {
  const [fen, setFen] = useState('');
  const [error, setError] = useState('');

  const handleInputChange = (e) => {
    const value = e.target.value;
    setFen(value);
    
    // Basic FEN validation: standard FEN strings must have 6 space-separated fields
    if (value.trim() && value.trim().split(/\s+/).length !== 6) {
      setError('Invalid FEN: must contain 6 fields');
    } else {
      setError('');
    }
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    if (!error && fen.trim() && onLoadFen) {
      onLoadFen(fen.trim());
    }
  };

  return (
    <form onSubmit={handleSubmit} className="w-full max-w-md p-4 bg-white rounded-lg shadow border border-gray-200">
      <label htmlFor="fen-input" className="block text-sm font-medium text-gray-700 mb-1">
        FEN String
      </label>
      <div className="flex gap-2">
        <input
          id="fen-input"
          type="text"
          value={fen}
          onChange={handleInputChange}
          placeholder="rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
          className="flex-1 px-3 py-2 border border-gray-300 rounded-md text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-700"
        />
        <button
          type="submit"
          disabled={!!error || !fen.trim()}
          className="px-4 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Load
        </button>
      </div>
      {error && <p className="mt-1.5 text-xs text-red-600">{error}</p>}
    </form>
  );
}