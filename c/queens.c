#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

int board[8] = {-1};

_Bool check_square(const int row, const int col) {
    for (int i = 0; i < row; i++) {
        int j = board[i];
        assert(j > -1);
        if (j == col
            || j - i == col - row
            || j + i == col + row)
            return 0;
    }
    return 1;
}

void solve(const int row) {
    if (row > 7) {
        for (int i = 0; i < 8; i++) printf("%d ", board[i]);
        puts("");
        exit(EXIT_SUCCESS);
    }
    for (int col = 0; col < 8; col++) {
        if (!check_square(row, col)) continue;
        board[row] = col;
        solve(row + 1);
        board[row] = -1;
    }
}

int main(void) {
    solve(0);
}
