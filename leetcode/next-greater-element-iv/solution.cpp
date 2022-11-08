// link to the problem: https://leetcode.com/problems/next-greater-element-iv/
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

// Runtime: 735 ms, faster than 32.74% of C++ online submissions for Next Greater Element IV.
// Memory Usage: 168.8 MB, less than 8.23% of C++ online submissions for Next Greater Element IV.

class Solution {
 public:
  std::vector<std::vector<int>> nextGreaterElement(const std::vector<int>& nums,
                                                   const std::vector<std::vector<int>>& previousElements) {
    std::vector<std::vector<int>> answer(nums.size());
    std::vector<int> monotonicStack;
    for (int i = nums.size() - 1; i >= 0; --i) {
      for (auto j : previousElements.empty() ? std::vector<int>{i} : previousElements[i]) {
        auto found =
            std::upper_bound(monotonicStack.rbegin(), monotonicStack.rend(), j,
                             [&](const auto& first, const auto& second) { return nums[first] < nums[second]; });
        if (found != monotonicStack.rend()) answer[*found].push_back(j);
      }
      while (!monotonicStack.empty() && nums[monotonicStack.back()] < nums[i]) monotonicStack.pop_back();
      monotonicStack.push_back(i);
    }
    return answer;
  }
  std::vector<int> secondGreaterElement(std::vector<int>& nums) {
    std::vector<std::vector<int>> previous;
    for (int i = 0; i < 2; ++i) previous = nextGreaterElement(nums, previous);
    std::vector<int> answer(nums.size(), -1);
    for (int i = 0; i < nums.size(); i++) {
      for (const auto& j : previous[i]) answer[j] = nums[i];
    }
    return answer;
  };
};

int main(int argc, char** argv) {
  std::vector<std::vector<int>> cases;
  cases.push_back({2, 4, 0, 9, 6});
  Solution s;
  for (auto nums : cases) {
    auto answer = s.secondGreaterElement(nums);
    std::copy(answer.begin(), answer.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
  }
}
