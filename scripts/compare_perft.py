import pandas as pd

stockfish = pd.read_csv("test_results/stockfish.txt", sep=": ", header=None, names=['move', 'count'], dtype={'move': str, 'count': int}, engine='python')

my_engine = pd.read_csv("test_results/my_engine.txt", sep=": ", header=None, names=['move', 'count'], dtype={'move': str, 'count': int}, engine='python')

# Find all the moves that are in stockfish but not in my_engine
missing_moves = stockfish[~stockfish['move'].isin(my_engine['move'])]
print("~~Missing moves in my_engine~~")
print(missing_moves, "\n")

# Find all the moves that are in my_engine but not in stockfish
extra_moves = my_engine[~my_engine['move'].isin(stockfish['move'])]
print("~~Extra moves in my_engine~~")
print(extra_moves, "\n")

# Find all the common moves and the differrences between the counts, print any non-zero
common_moves = pd.merge(stockfish, my_engine, on='move', suffixes=('_stockfish', '_my_engine'))
common_moves['count_diff'] = common_moves['count_my_engine'] - common_moves['count_stockfish']
non_zero_diffs = common_moves[common_moves['count_diff'] != 0]
print("~~Common moves with non-zero count differences~~")
print(non_zero_diffs, "\n")