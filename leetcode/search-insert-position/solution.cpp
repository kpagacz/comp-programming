// link to the problem: https://leetcode.com/problems/search-insert-position/
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
  int searchInsert(std::vector<int>& nums, int target) {
    int l = 0, r = nums.size() - 1;
    while (l < r) {
      int middle = (l + r) / 2;
      if (nums[middle] == target) return middle;
      if (target < nums[middle]) {
        r = middle;
      } else {
        l = middle + 1;
      }
    }
    if (nums[l] < target) return l + 1; else
      return l;
  }
};

int main(int argc, char** argv) {
  std::vector<int> nums{1,7};
  int target = 2;
  Solution s;
  std::cout << "target: " << target << " index: " << s.searchInsert(nums, target) << '\n';
  target = 0;
  std::cout << "target: " << target << " index: " << s.searchInsert(nums, target) << '\n';
  target = 2;
  std::cout << "target: " << target << " index: " << s.searchInsert(nums, target) << '\n';
  target = 6;
  std::cout << "target: " << target << " index: " << s.searchInsert(nums, target) << '\n';
  target = 8;
  std::cout << "target: " << target << " index: " << s.searchInsert(nums, target) << '\n';
}
