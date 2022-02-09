// link to the problem: https://leetcode.com/problems/contiguous-array/
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
  int findMaxLength(std::vector<int>& nums) {
    std::unordered_map<int, int> prefix_sums;
    int running_sum = 0;
    prefix_sums.insert({0, 0});
    int max_length = 0;
    for (auto i{0}; i < nums.size(); i++) {
      running_sum += nums[i] == 0 ? -1 : 1;
      auto inserted = prefix_sums.insert({running_sum, i + 1});
      if (!inserted.second) max_length = std::max(max_length, i + 1 - inserted.first->second);
    }
    return max_length;
  }
};

int main(int argc, char** argv) {}
