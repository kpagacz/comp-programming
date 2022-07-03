// link to the problem: https://leetcode.com/problems/transpose-matrix/
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

class Solution {
 public:
  std::vector<std::vector<int>> transpose(std::vector<std::vector<int>>& matrix) {
    std::vector<std::vector<int>> answer(matrix[0].size(), std::vector<int>(matrix.size()));
    for (int i = 0; i < matrix.size(); ++i)
      for (int j = 0; j < matrix[0].size(); ++j) answer[j][i] = std::move(matrix[i][j]);
      return answer;
  }
};

int main(int argc, char** argv) {}
