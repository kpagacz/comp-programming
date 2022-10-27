// link to the problem: https://leetcode.com/problems/continuous-subarray-sum/
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

// Runtime: 251 ms, faster than 89.48% of C++ online submissions for Continuous Subarray Sum.
// Memory Usage: 113.9 MB, less than 46.54% of C++ online submissions for Continuous Subarray Sum.

class Solution {
 public:
  bool checkSubarraySum(std::vector<int>& nums, int k) {
    std::unordered_map<int, int> prefixSums;
    prefixSums.insert({0, 0});
    int runningSum = 0;
    for (int i = 0; i < nums.size(); ++i) {
      const auto& num = nums[i];
      runningSum += num;
      runningSum = runningSum % k;
      if (prefixSums.count(runningSum) == 0)
        prefixSums.insert({runningSum, i + 1});
      else if ((i + 1) - prefixSums[runningSum] >= 2)
        return true;
    }

    return false;
  }
};

int main(int argc, char** argv) {}
