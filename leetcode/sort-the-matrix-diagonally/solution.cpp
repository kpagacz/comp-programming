// link to the problem: https://leetcode.com/problems/sort-the-matrix-diagonally/
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

// Runtime: 19 ms, faster than 49.33% of C++ online submissions for Sort the Matrix Diagonally.
// Memory Usage: 9.3 MB, less than 52.67% of C++ online submissions for Sort the Matrix Diagonally.

class Solution {
 public:
  std::vector<std::vector<int>> diagonalSort(std::vector<std::vector<int>>& mat) {
    for (int row = 0; row < mat.size(); ++row) {
      std::vector<int> temp;
      for (int i = row, j = 0; i < mat.size() && j < mat[0].size(); ++i, ++j) {
        temp.push_back(mat[i][j]);
      }
      std::sort(temp.begin(), temp.end());
      for (int i = row, j = 0; i < mat.size() && j < mat[0].size(); ++i, ++j) mat[i][j] = temp[i - row];
    }

    for (int col = 0; col < mat[0].size(); ++col) {
      std::vector<int> temp;
      for (int i = 0, j = col; i < mat.size() && j < mat[0].size(); ++i, ++j) temp.push_back(mat[i][j]);
      std::sort(temp.begin(), temp.end());
      for (int i = 0, j = col; i < mat.size() && j < mat[0].size(); ++i, ++j) mat[i][j] = temp[j - col];
    }

    return mat;
  }
};
int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<std::vector<int>>> tests;
  tests.push_back({{3, 3, 1, 1}, {2, 2, 1, 2}, {1, 1, 1, 2}});
  for (auto& test : tests) {
    const auto res = s.diagonalSort(test);
    for (const auto& row : res) {
      std::copy(row.begin(), row.end(), std::ostream_iterator<int>(std::cout, " "));
      std::cout << '\n';
    }
  }
}
