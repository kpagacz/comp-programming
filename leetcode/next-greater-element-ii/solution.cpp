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

class Solution {
 public:
  std::vector<int> nextGreaterElements(std::vector<int>& nums) {
    std::stack<int> monotonicStack;
    nums.reserve(nums.size() * 2);
    for (const auto& el : nums) nums.push_back(el);
    std::unordered_map<int, int> nextGreaterMap;
    for (int i = 0; i < nums.size(); ++i) {
      // std::cout << "num: " << nums[i] << '\n';
      while (!monotonicStack.empty() && nums[i] > nums[monotonicStack.top()]) {
        nextGreaterMap.insert({monotonicStack.top(), i});
        monotonicStack.pop();
      }

      // for (const auto& [key, value] : nextGreaterMap) std::cout << "key: " << key << " value: " << value << '\n';
      monotonicStack.push(i);
    }

    std::vector<int> answer;
    for (int i = 0; i < nums.size() / 2; ++i) {
      const auto nextIndex = nextGreaterMap.count(i) == 1 ? nextGreaterMap[i] : -1;
      answer.push_back(nextIndex == -1 ? -1 : nums[nextIndex]);
    }

    return answer;
  }
};

int main(int argc, char** argv) {
  // std::vector<int> v{100, 1, 11, 1, 120, 111, 123, 1, -1, -100};
  // std::vector<int> v{1,2,1};
  std::vector<int> v{1,2,3,4,3};
  Solution s;
  s.nextGreaterElements(v);
}
