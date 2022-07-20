// link to the problem:  https://leetcode.com/problems/search-in-rotated-sorted-array/
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

// Runtime: 5 ms, faster than 59.84% of C++ online submissions for Search in Rotated Sorted Array.
// Memory Usage: 11.1 MB, less than 74.87% of C++ online submissions for Search in Rotated Sorted Array.

class Solution {
 public:
  int search(std::vector<int>& nums, int target) {
    int left = 0, right = nums.size() - 1;
    while (right != left) {
      int middle = (right + left) / 2;
      if (nums[middle] >= nums.front()) {
        left = middle + 1;
      } else {
        right = middle;
      }
    }

    auto foundFirstPart = std::lower_bound(nums.begin(), nums.begin() + left, target);
    if (foundFirstPart != nums.begin() + left && *foundFirstPart == target)
      return std::distance(nums.begin(), foundFirstPart);
    auto foundSecondPart = std::lower_bound(nums.begin() + left, nums.end(), target);
    if (foundSecondPart != nums.end() && *foundSecondPart == target)
      return std::distance(nums.begin(), foundSecondPart);
    return -1;
  }
};

int main(int argc, char** argv) {}
