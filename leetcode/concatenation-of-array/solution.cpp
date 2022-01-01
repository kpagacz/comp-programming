// link to the problem: https://leetcode.com/problems/concatenation-of-array/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <string>
#include <vector>

class Solution {
 public:
  std::vector<int> getConcatenation(std::vector<int>& nums) {
    int old_size = nums.size();
    nums.resize(2 * nums.size());
    std::copy(nums.begin(), std::next(nums.begin(), old_size), std::next(nums.begin(), old_size));
    return nums;
  }
};

int main(int argc, char** argv) {}
