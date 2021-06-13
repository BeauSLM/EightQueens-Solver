# EightQueens.py
# Beau McCartney
# June 2021

def init_file():
    return open('lib/' + input('Choose a puzzle: ') + '.txt')

def init_board():
    board = [[0 for i in range(8)] for j in range(8)]
    fhand = init_file()

    i = 0
    j = 0
    for line in fhand:
        line = line.strip()
        for number in line:
            board[i][j] = int(number)
            j += 1
        i =+ 1
        j = 0
    fhand.close()
    print("Puzzle: ")
    print_board(board)
    return board

def print_board(board):
    for row in board:
        print(row)

def check_if_placeable(board, row, column):
    for i in range(8):
        for j in range(8):
            if i == row and j == column:
                continue
            if (i == row or j == column) and board[i][j]:
                return False
            if (i - j == row - column or i + j == row + column) and board[i][j]:
                return False
    return True

def check_valid(board):
    for i in range(8):
        for j in range(8):
            if board[i][j]:
                if not check_if_placeable(board, i, j):
                    return False
    return True

class calls:
    calls = 0

call_track = calls()

def solve_eightqueens(board, number):
    call_track.calls += 1
    for row in range(8):
        for column in range(8):
            if board[row][column]:
                continue
            if not check_if_placeable(board, row, column):
                continue
            board[row][column] = 1
            number -= 1
            if number == 0:
                return 0
            if solve_eightqueens(board, number) == 0:
                return 0
            board[row][column] = 0
            number += 1

if __name__ == "__main__":
    board = init_board()
    number = int(input("How many queens would you like to place?\n"))
    solve_eightqueens(board, number)
    print("\nFinal board:")
    print_board(board)
    print(call_track.calls, "number of calls")
