// link to the problem: https://leetcode.com/problems/remove-duplicates-from-sorted-array/
#include <iostream>
#include <sstream>
#include <vector>

// Runtime: 16 ms, faster than 82.31% of C++ online submissions for Remove Duplicates from Sorted Array.
// Memory Usage: 18.5 MB, less than 37.30% of C++ online submissions for Remove Duplicates from Sorted Array.

class Solution {
 public:
  int removeDuplicates(std::vector<int>& nums) {
    auto it = nums.begin(), unique = nums.begin();
    while (++it != nums.end())
      if (*it != *unique) *(++unique) = *it;
    return unique - nums.begin() + 1;
  }
};

int main() {
  Solution s;
  std::vector<std::vector<int>> cases;
  cases.push_back({1, 2, 3, 4});
  cases.push_back({1, 1, 1, 2, 4, 5});
  cases.push_back({1, 1, 2, 2});
  for (auto nums : cases) {
    std::copy(nums.begin(), nums.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
    std::cout << s.removeDuplicates(nums) << '\n';
  }
}
