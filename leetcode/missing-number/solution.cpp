// link to the problem: https://leetcode.com/problems/missing-number/
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

// Runtime: 32 ms, faster than 50.81% of C++ online submissions for Missing Number.
// Memory Usage: 17.9 MB, less than 64.62% of C++ online submissions for Missing Number.

class Solution {
 public:
  int missingNumber(std::vector<int>& nums) {
    return nums.size() * (nums.size() + 1) / 2 - std::accumulate(nums.begin(), nums.end(), 0);
  }
};

int main(int argc, char** argv) {}
