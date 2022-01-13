// link to the problem:
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

class Solution {
 public:
  int removeDuplicates(std::vector<int>& nums) {
    auto it = nums.begin();
    auto pred = it;
    if (it == nums.end()) return std::distance(nums.begin(), nums.end());
    while (++it != nums.end()) {
      if (!(*it == *pred) && ++pred != it) *pred = std::move(*it);
    }

    return std::distance(nums.begin(), pred) + 1;
  }
};

int main(int argc, char** argv) {}
