// link to the problem: https://leetcode.com/problems/number-of-islands/
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

// Runtime: 47 ms, faster than 69.24% of C++ online submissions for Number of Islands.
// Memory Usage: 12.5 MB, less than 44.10% of C++ online submissions for Number of Islands.

class Solution {
 public:
  const char GUARDIAN = 'x';
  int rows, cols;
  int numIslands(std::vector<std::vector<char>>& grid) {
    rows = grid.size();
    cols = grid[0].size();
    int answer = 0;
    for (int i = 0; i < rows; ++i)
      for (int j = 0; j < cols; ++j)
        if (grid[i][j] == '1') {
          ++answer;
          floodFill(i, j, grid);
        }
    return answer;
  }

  void floodFill(const int& row, const int& col, std::vector<std::vector<char>>& grid) {
    if (row < 0 || row >= rows || col < 0 || col >= cols) return;
    if (grid[row][col] == GUARDIAN || grid[row][col] == '0') return;
    grid[row][col] = GUARDIAN;
    floodFill(row - 1, col, grid);
    floodFill(row, col - 1, grid);
    floodFill(row, col + 1, grid);
    floodFill(row + 1, col, grid);
  }
};

int main(int argc, char** argv) {}
