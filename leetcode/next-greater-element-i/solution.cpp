// link to the problem: https://leetcode.com/problems/next-greater-element-i/
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


// Runtime: 4 ms, faster than 89.84% of C++ online submissions for Next Greater Element I.
// Memory Usage: 8.9 MB, less than 57.74% of C++ online submissions for Next Greater Element I.

class Solution {
 public:
  std::vector<int> nextGreaterElement(std::vector<int>& nums1, std::vector<int>& nums2) {
    std::stack<int> monotonicStack;
    std::unordered_map<int, int> nextGreaterElements;
    for (const auto& num : nums2) {
      while (!monotonicStack.empty() && num > monotonicStack.top()) {
        nextGreaterElements.insert({monotonicStack.top(), num});
        monotonicStack.pop();
      }
      monotonicStack.push(num);
    }
    std::vector<int> answer;
    for (const auto& num : nums1) {
      int nextGreaterElement = nextGreaterElements.count(num) == 1 ? nextGreaterElements[num] : -1;
      answer.push_back(nextGreaterElement);
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
