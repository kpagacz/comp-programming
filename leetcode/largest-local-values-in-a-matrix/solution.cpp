// link to the problem: https://leetcode.com/problems/largest-local-values-in-a-matrix/
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

// Runtime: 34 ms, faster than 18.35% of C++ online submissions for Largest Local Values in a Matrix.
// Memory Usage: 11.1 MB, less than 43.39% of C++ online submissions for Largest Local Values in a Matrix.

class Solution {
 public:
  std::vector<std::vector<int>> largestLocal(std::vector<std::vector<int>>& grid) {
    const int& size = grid.size();
    std::vector<std::vector<int>> answer(size - 2, std::vector<int>(size - 2));
    const std::vector<std::pair<int, int>> cardinals{{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 0},
                                                     {0, 1},   {1, -1}, {1, 0},  {1, 1}};
    for (int row = 1; row < size - 1; ++row)
      for (int col = 1; col < size - 1; ++col) {
        int localMax = INT32_MIN;
        for (const auto& [x, y] : cardinals) localMax = std::max(localMax, grid[row + x][col + y]);
        answer[row - 1][col - 1] = localMax;
      }
    return answer;
  }
};

int main(int argc, char** argv) {}
