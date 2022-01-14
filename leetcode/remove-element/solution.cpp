// link to the problem: https://leetcode.com/problems/remove-element/
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
#include <vector>

class Solution {
 public:
  int removeElement(std::vector<int>& nums, int val) {
    auto first = std::find(nums.begin(), nums.end(), val);
    if (first == nums.end()) return nums.size();
    for (auto it = first; ++it != nums.end();) {
      if (*it != val) *first++ = *it;
    }

    return std::distance(nums.begin(), first);
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> v{1, 2, 3};
  s.removeElement(v, 1);
}
