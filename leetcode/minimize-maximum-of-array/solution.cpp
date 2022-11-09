// link to the problem: https://leetcode.com/problems/minimize-maximum-of-array/
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

// Runtime: 423 ms, faster than 32.01% of C++ online submissions for Minimize Maximum of Array.
// Memory Usage: 71.4 MB, less than 56.51% of C++ online submissions for Minimize Maximum of Array.

class Solution {
 public:
  int minimizeArrayValue(std::vector<int>& nums) {
    int64_t runningSum = 0;
    int currentMax = INT32_MIN;

    for (int i = 0; i < nums.size(); ++i) {
      runningSum += nums[i];
      if (nums[i] >= currentMax) currentMax = std::max(1.0 * currentMax, std::ceil(1.0 * runningSum / (i + 1)));
    }
    return currentMax;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<int>> cases;
  cases.push_back({3, 7, 1, 6});
  cases.push_back({10, 1});
  cases.push_back({4, 5, 10, 1});
  cases.push_back({4, 7, 2, 2, 9, 19, 16, 0, 3, 15});
  cases.push_back({0, 6});
  for (auto nums : cases) std::cout << s.minimizeArrayValue(nums) << '\n';
}
