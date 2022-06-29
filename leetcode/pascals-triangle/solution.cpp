// link to the problem: https://leetcode.com/problems/pascals-triangle/
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

// Results:
// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Pascal's Triangle.
// Memory Usage: 6.5 MB, less than 32.08% of C++ online submissions for Pascal's Triangle.

class Solution {
 public:
  std::vector<std::vector<int>> generate(int numRows) {
    std::vector<std::vector<int>> answer(numRows);
    for (int i = 0; i < answer.size(); ++i) {
      answer[i] = std::vector<int>(i + 1);
      answer[i].front() = answer[i].back() = 1;
      for (int j = 1; j < answer[i].size() - 1; ++j) answer[i][j] = answer[i - 1][j - 1] + answer[i - 1][j];
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
