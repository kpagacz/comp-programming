// link to the problem: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
 public:
  int removeDuplicates(std::vector<int>& nums) {
    auto it = nums.begin(), sorted = nums.begin();
    int counter = 0;
    while (++it != nums.end()) {
      if (*sorted != *it) {
        if (++sorted != it) *sorted = std::move(*it);
        counter = 0;
      } else {
        counter++;
        if (counter < 2 && ++sorted != it) *sorted = std::move(*it);
      }
    }
    return std::distance(nums.begin(), sorted) + 1;
  }
};

int main(int argc, char** argv) {
  std::vector<int> nums{1, 1, 2, 2, 2, 2, 2, 3};
  Solution s;
  auto k = s.removeDuplicates(nums);
  for (int i = 0; i < k; i++) std::cout << nums[i] << " ";
  std::cout << "\n";
}
