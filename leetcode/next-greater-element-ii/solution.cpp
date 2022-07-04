// link to the problem: https://leetcode.com/problems/next-greater-element-ii/
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

// Runtime: 42 ms, faster than 79.46% of C++ online submissions for Next Greater Element II.
// Memory Usage: 24 MB, less than 62.69% of C++ online submissions for Next Greater Element II.

class Solution {
 public:
  std::vector<int> nextGreaterElements(std::vector<int>& nums) {
    std::stack<int> monotonicStack;
    std::vector<int> nextGreaterMap(nums.size(), -1);
    for (int j = 0; j < 2; ++j)
      for (int i = 0; i < nums.size(); ++i) {
        // std::cout << "num: " << nums[i] << '\n';
        while (!monotonicStack.empty() && nums[i] > nums[monotonicStack.top()]) {
          nextGreaterMap[monotonicStack.top()] = i;
          monotonicStack.pop();
        }

        // for (const auto& [key, value] : nextGreaterMap) std::cout << "key: " << key << " value: " << value << '\n';
        monotonicStack.push(i);
      }

    for (int i = 0; i < nextGreaterMap.size(); ++i) {
      if (nextGreaterMap[i] != -1) nextGreaterMap[i] = nums[nextGreaterMap[i]];
    }

    return nextGreaterMap;
  }
};

int main(int argc, char** argv) {
  // std::vector<int> v{100, 1, 11, 1, 120, 111, 123, 1, -1, -100};
  // std::vector<int> v{1,2,1};
  std::vector<int> v{1, 2, 3, 4, 3};
  Solution s;
  s.nextGreaterElements(v);
}
