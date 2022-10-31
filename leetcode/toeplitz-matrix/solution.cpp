// link to the problem: https://leetcode.com/problems/toeplitz-matrix/
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

// Runtime: 26 ms, faster than 68.65% of C++ online submissions for Toeplitz Matrix.
// Memory Usage: 17.5 MB, less than 63.25% of C++ online submissions for Toeplitz Matrix.

class Solution {
 public:
  bool isToeplitzMatrix(std::vector<std::vector<int>>& matrix) {
    for (int i = 0; i < matrix[0].size(); ++i) {
      int match = matrix[0][i];
      for (int j = 0; j < matrix.size() && i + j < matrix[0].size(); ++j)
        if (matrix[j][i + j] != match) return false;
    }

    for (int row = 0; row < matrix.size(); ++row) {
      int match = matrix[row][0];
      for (int i = 0; i + row < matrix.size() && i < matrix[0].size(); ++i)
        if (matrix[i + row][i] != match) return false;
    }
    return true;
  }
};

int main(int argc, char** argv) {}
