// link to the problem: https://leetcode.com/problems/search-a-2d-matrix-ii/
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
  bool searchMatrix(std::vector<std::vector<int>>& matrix, int target) {
    const auto& lowerBound =
        std::lower_bound(matrix.begin(), matrix.end(), target,
                         [](const auto& candidate, const auto& target) { return candidate[0] <= target; });
    bool answer = false;
    std::for_each(matrix.begin(), lowerBound,
                  [&](const auto& row) { answer = answer || std::binary_search(row.begin(), row.end(), target); });
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<int>> v{
      {1, 4, 7, 11, 15}, {2, 5, 8, 12, 19}, {3, 6, 9, 16, 22}, {10, 13, 14, 17, 24}, {18, 21, 23, 26, 30}};
  int target = 5;
  std::vector<int> targets{5, 20};
  for (const auto& target : targets) {
    std::cout << std::boolalpha << s.searchMatrix(v, target) << "\n";
  }
}
