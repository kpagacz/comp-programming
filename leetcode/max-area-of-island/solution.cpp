// link to the problem: https://leetcode.com/problems/max-area-of-island/
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

// Runtime: 21 ms, faster than 84.06% of C++ online submissions for Max Area of Island.
// Memory Usage: 23.5 MB, less than 43.62% of C++ online submissions for Max Area of Island.

class Solution {
 public:
  int maxAreaOfIsland(std::vector<std::vector<int>>& grid) {
    int answer = 0;
    for (int row = 0; row < grid.size(); ++row)
      for (int col = 0; col < grid[0].size(); ++col) answer = std::max(answer, floodFill(grid, row, col));
    return answer;
  }

 private:
  int floodFill(std::vector<std::vector<int>>& grid, const int& row, const int& col) {
    if (row < 0 || row >= grid.size() || col < 0 || col >= grid[0].size()) return 0;
    if (grid[row][col] != 1) return 0;
    grid[row][col] = 2;
    return 1 + floodFill(grid, (row - 1), col) + floodFill(grid, (row + 1), col) + floodFill(grid, row, (col - 1)) +
           floodFill(grid, row, (col + 1));
  }
};

int main(int argc, char** argv) {}
