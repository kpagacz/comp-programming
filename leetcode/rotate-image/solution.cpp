// link to the problem: https://leetcode.com/problems/rotate-image/
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

// Runtime: 3 ms, faster than 64.32% of C++ online submissions for Rotate Image.
// Memory Usage: 7.2 MB, less than 33.50% of C++ online submissions for Rotate Image.

class Solution {
 public:
  void rotate(std::vector<std::vector<int>>& matrix) {
    for (int i = 0; i < matrix.size() / 2; ++i)
      for (int j = 0; j < (matrix[0].size() + 1) / 2; ++j) {
        // std::cout << i << " " << j << '\n';
        // std::cout << j << " " << matrix.size() - i - 1 << '\n';
        // std::cout << matrix.size() - 1 - j << " " << i << '\n';
        // std::cout << matrix.size() - 1 - i << " " << matrix[0].size() - 1 - j << '\n';
        std::swap(matrix[i][j], matrix[j][matrix.size() - 1 - i]);
        std::swap(matrix[i][j], matrix[matrix.size() - 1 - j][i]);
        std::swap(matrix[matrix.size() - 1 - j][i], matrix[matrix.size() - 1 - i][matrix[0].size() - 1 - j]);
      }
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<std::vector<int>>> tests;
  tests.push_back({{1, 2, 3}, {4, 5, 6}, {7, 8, 9}});
  tests.push_back({{5, 1, 9, 11}, {2, 4, 8, 10}, {13, 3, 6, 7}, {15, 14, 12, 16}});
  for (auto& test : tests) {
    for (const auto& row : test) {
      std::copy(row.begin(), row.end(), std::ostream_iterator<int>(std::cout, " "));
      std::cout << '\n';
    }
    s.rotate(test);
    for (const auto& row : test) {
      std::copy(row.begin(), row.end(), std::ostream_iterator<int>(std::cout, " "));
      std::cout << '\n';
    }
  }
}
