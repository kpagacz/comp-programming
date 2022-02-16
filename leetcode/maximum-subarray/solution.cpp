// link to the problem: https://leetcode.com/problems/maximum-subarray/
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
    int maxSubArray(std::vector<int>& nums) {
      // Kadane's
      int current_sum = 0, max_sum = INT32_MIN;
      for(const auto& num : nums) {
        current_sum += num;
        max_sum = std::max(max_sum, current_sum);
        current_sum = std::max(0, current_sum);
      }

      return max_sum;
    }
};

int main(int argc, char** argv) {}
