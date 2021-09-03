# unit_test.py
# Beau McCartney
# June 2021

import unittest
import EightQueens

class TestHelperMethods(unittest.TestCase):

    def test_check_if_placeable_empty(self):
        board = [[0 for i in range(8)] for j in range(8)]
        self.assertTrue(EightQueens.check_if_placeable(board, 3, 3))

    def test_check_if_placeable_self_filled(self):
        board = [[0 for i in range(8)] for j in range(8)]
        board[3][3] = 1
        self.assertTrue(EightQueens.check_if_placeable(board, 3, 3))

    def test_check_if_placeable_row(self):
        board = [[0 for i in range(8)] for j in range(8)]
        board[3][4] = 1
        self.assertFalse(EightQueens.check_if_placeable(board, 3, 3))

    def test_check_if_placeable_column(self):
        board = [[0 for i in range(8)] for j in range(8)]
        board[4][3] = 1
        self.assertFalse(EightQueens.check_if_placeable(board, 3, 3))

    def test_check_if_placeable_diagnol(self):
        board = [[0 for i in range(8)] for j in range(8)]
        board[5][5] = 1
        self.assertFalse(EightQueens.check_if_placeable(board, 3, 3))
        
if __name__ == "__main__":
    unittest.main()
