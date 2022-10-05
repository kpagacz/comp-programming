// link to the problem: https://leetcode.com/problems/maximum-sum-of-an-hourglass/
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

// Runtime: 125 ms, faster than 16.67% of C++ online submissions for Maximum Sum of an Hourglass.
// Memory Usage: 13.2 MB, less than 83.33% of C++ online submissions for Maximum Sum of an Hourglass.

class Solution {
 public:
  int maxSum(std::vector<std::vector<int>>& grid) {
    int answer = 0;
    for (int row = 1; row < grid.size() - 1; ++row)
      for (int col = 1; col < grid[0].size() - 1; ++col) {
        int hourglassSum = 0;
        hourglassSum += grid[row][col];
        for (int i : {-1, 0, 1}) hourglassSum += grid[row - 1][col + i] + grid[row + 1][col + i];
        answer = std::max(answer, hourglassSum);
      }
    return answer;
  }
};

int main(int argc, char** argv) {}
