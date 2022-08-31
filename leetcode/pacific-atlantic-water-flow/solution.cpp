// link to the problem: https://leetcode.com/problems/pacific-atlantic-water-flow/
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

// I think complexity-wise I am fine both time and space-wise. I have some large constants in there and
// that is why my solution runs so slow...

// Runtime: 286 ms, faster than 10.05% of C++ online submissions for Pacific Atlantic Water Flow.
// Memory Usage: 39.8 MB, less than 7.32% of C++ online submissions for Pacific Atlantic Water Flow.

constexpr int MASK = (1 << 20) - 1;
constexpr int PACIFIC_BIT = 21, ATLANTIC_BIT = 22;

class Solution {
 public:
  std::vector<std::vector<int>> pacificAtlantic(std::vector<std::vector<int>>& heights) {
    for (int i = 0; i < heights[0].size(); ++i) {
      floodFill(0, i, heights, PACIFIC_BIT);
      floodFill(heights.size() - 1, i, heights, ATLANTIC_BIT);
    }
    for (int i = 0; i < heights.size(); ++i) {
      floodFill(i, 0, heights, PACIFIC_BIT);
      floodFill(i, heights[0].size() - 1, heights, ATLANTIC_BIT);
    }
    std::vector<std::vector<int>> answer;
    for (int i = 0; i < heights.size(); ++i)
      for (int j = 0; j < heights[0].size(); ++j)
        if ((heights[i][j] >> PACIFIC_BIT) & 1 && (heights[i][j] >> ATLANTIC_BIT) & 1) answer.push_back({i, j});

    return answer;
  }

  int height(const int& row, const int& col, const std::vector<std::vector<int>>& heights) {
    return heights[row][col] & MASK;
  }

  void floodFill(const int& row, const int& col, std::vector<std::vector<int>>& heights, const int& oceanBit) {
    if ((heights[row][col] >> oceanBit) & 1) return;
    heights[row][col] += 1 << oceanBit;

    for (const auto& direction : std::vector<std::vector<int>>{{-1, 0}, {0, -1}, {1, 0}, {0, 1}}) {
      const int &newRow = row + direction[0], &newCol = col + direction[1];
      if (newRow < 0 || newRow >= heights.size() || newCol < 0 || newCol >= heights[0].size()) continue;
      if (height(row, col, heights) <= height(newRow, newCol, heights)) floodFill(newRow, newCol, heights, oceanBit);
    }
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<std::vector<int>>> tests;
  tests.push_back({{1, 2, 2, 3, 5}, {3, 2, 3, 4, 4}, {2, 4, 5, 3, 1}, {6, 7, 1, 4, 5}, {5, 1, 1, 2, 4}});
  tests.push_back({{1}});
  tests.push_back({{10, 10, 10}, {10, 1, 10}, {10, 10, 10}});
  tests.push_back({{10, 10, 10}, {10, 10, 10}, {10, 10, 10}});
  tests.push_back({{10, 10, 10, 10}, {10, 10, 10, 10}, {10, 10, 10, 10}, {10, 10, 10, 10}});
  for (auto& test : tests) {
    std::cout << "TEST RESULT:\n";
    const auto answer = s.pacificAtlantic(test);
    for (const auto& row : answer) {
      std::copy(row.begin(), row.end(), std::ostream_iterator<int>(std::cout, " "));
      std::cout << '\n';
    }
  }
}
