// link to the problem: https://leetcode.com/problems/valid-sudoku/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

// Runtime: 43 ms, faster than 61.69% of C++ online submissions for Valid Sudoku.
// Memory Usage: 18.4 MB, less than 61.65% of C++ online submissions for Valid Sudoku.

class Solution {
 public:
  bool isValidSudoku(std::vector<std::vector<char>>& board) {
    // rows and columns
    // std::cout << "Rows and columns\n";
    for (int i = 0; i < 9; i++) {
      std::vector<bool> digits(9, false);
      for (int j = 0; j < 9; j++) {
        // std::printf("i: %d j: %d\n", i, j);
        if (board[i][j] != '.') {
          if (digits[board[i][j] - '1'] == false) digits[board[i][j] - '1'] = true;
          else return false;
        }
      }
      std::fill(digits.begin(), digits.end(), false);
      for (int j = 0; j < 9; j++)
        if (board[j][i] != '.') {
          if (digits[board[j][i] - '1'] == false) digits[board[j][i] - '1'] = true;
          else return false;
        }
    }

    // small squares
    // std::cout << "Small squares\n";
    const std::vector<std::pair<int, int>> cardinalDirections{{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 0},
                                                              {0, 1},   {1, -1}, {1, 0},  {1, 1}};
    for (int i = 1; i < 9; i += 3)
      for (int j = 1; j < 9; j += 3) {
        std::vector<bool> digits(9, false);
        for (const auto& [deltaX, deltaY] : cardinalDirections)
          if (board[i + deltaX][j + deltaY] != '.') {
            if (digits[board[i + deltaX][j + deltaY] - '1'] == false)
              digits[board[i + deltaX][j + deltaY] - '1'] = true;
            else return false;
          }
      }
    return true;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<std::vector<char>>> cases;
  cases.push_back({{'5', '3', '.', '.', '7', '.', '.', '.', '.'},
                   {'6', '.', '.', '1', '9', '5', '.', '.', '.'},
                   {'.', '9', '8', '.', '.', '.', '.', '6', '.'},
                   {'8', '.', '.', '.', '6', '.', '.', '.', '3'},
                   {'4', '.', '.', '8', '.', '3', '.', '.', '1'},
                   {'7', '.', '.', '.', '2', '.', '.', '.', '6'},
                   {'.', '6', '.', '.', '.', '.', '2', '8', '.'},
                   {'.', '.', '.', '4', '1', '9', '.', '.', '5'},
                   {'.', '.', '.', '.', '8', '.', '.', '7', '9'}});
  cases.push_back({{'.', '.', '.', '.', '5', '.', '.', '1', '.'},
                   {'.', '4', '.', '3', '.', '.', '.', '.', '.'},
                   {'.', '.', '.', '.', '.', '3', '.', '.', '1'},
                   {'8', '.', '.', '.', '.', '.', '.', '2', '.'},
                   {'.', '.', '2', '.', '7', '.', '.', '.', '.'},
                   {'.', '1', '5', '.', '.', '.', '.', '.', '.'},
                   {'.', '.', '.', '.', '.', '2', '.', '.', '.'},
                   {'.', '2', '.', '9', '.', '.', '.', '.', '.'},
                   {'.', '.', '4', '.', '.', '.', '.', '.', '.'}});
  for (auto board : cases) std::cout << std::boolalpha << s.isValidSudoku(board) << '\n';
}
