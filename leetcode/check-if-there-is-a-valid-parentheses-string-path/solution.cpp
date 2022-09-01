// link to the problem: https://leetcode.com/problems/check-if-there-is-a-valid-parentheses-string-path/
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

// Runtime: 135 ms, faster than 91.63% of C++ online submissions for Check if There Is a Valid Parentheses String Path.
// Memory Usage: 17.6 MB, less than 85.26% of C++ online submissions for Check if There Is a Valid Parentheses String
// Path.

class Solution {
 public:
  int visited[100][100][101] = {};
  bool hasValidPath(std::vector<std::vector<char>>& grid, int i = 0, int j = 0, int bal = 0) {
    if (i >= grid.size() || j >= grid[0].size()) return false;
    bal += grid[i][j] == '(' ? 1 : -1;
    if (bal < 0 || bal > (grid.size() + grid[0].size()) / 2 || visited[i][j][bal]) return false;
    visited[i][j][bal] = true;
    if (i == grid.size() - 1 && j == grid[0].size() - 1 && bal == 0) return true;
    return hasValidPath(grid, i + 1, j, bal) || hasValidPath(grid, i, j + 1, bal);
  }
};

int main(int argc, char** argv) {
  std::vector<std::vector<std::vector<char>>> tests;
  tests.push_back({{'(', ')'}, {'(', '}'}});
  Solution s;
  for (auto& test : tests) {
    std::cout << std::boolalpha << s.hasValidPath(test) << '\n';
  }
}
