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

// Runtime: 10 ms, faster than 72.38% of C++ online submissions for Find First and Last Position of Element in Sorted
// Array. Memory Usage: 13.6 MB, less than 63.05% of C++ online submissions for Find First and Last Position of Element
// in Sorted Array.

class Solution {
 public:
  std::vector<int> searchRange(std::vector<int>& nums, int target) {
    std::vector<int> answer(2, -1);
    if (nums.empty()) return answer;
    if (target < nums.front() || target > nums.back()) return answer;
    int left = 0, right = nums.size();
    while (left != right) {
      int middle = (right + left) / 2;
      if (nums[middle] == target && (middle == 0 || nums[middle - 1] != target)) {
        answer[0] = middle;
        break;
      }
      if (nums[middle] < target)
        left = middle + 1;
      else
        right = middle;
    }

    // std::cout << "rangeBegin: " << answer[0] << '\n';

    left = answer[0];
    right = nums.size();

    while (left != right) {
      int middle = (right + left) / 2;
      if (nums[middle] == target && (middle + 1 == nums.size() || nums[middle + 1] != target)) {
        answer[1] = middle;
        break;
      }

      if (nums[middle] > target)
        right = middle;
      else
        left = middle + 1;
    }

    // std::cout << "rangeEnd: " << answer[1] << '\n';
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<int>> nums{{1, 2, 2, 2, 3}, {2, 2, 2, 2, 3}, {2}, {2, 2}, {1, 1, 1, 1, 2}, {1}};
  for (auto& test : nums) s.searchRange(test, 0);
}
