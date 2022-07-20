// link to the problem: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
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
  std::vector<int> searchRange(std::vector<int>& nums, int target) {
    std::vector<int> answer(2, -1);
    auto lowerBound = std::lower_bound(nums.begin(), nums.end(), target);
    if (lowerBound == nums.end() || *lowerBound != target) return answer;
    answer[0] = std::distance(nums.begin(), lowerBound);

    auto rangeEnd = std::lower_bound(lowerBound, nums.end(), target + 1);
    answer[1] = std::distance(nums.begin(), rangeEnd) - 1;
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<int>> nums{{1, 2, 2, 2, 3}, {2, 2, 2, 2, 3}, {2}, {2, 2}, {1, 1, 1, 1, 2}, {1}};
  for (auto& test : nums) {
    const auto &v = s.searchRange(test, 0);
    std::cout << "Begin: " << v[0] << " End: " << v[1] << '\n';
  }
}
