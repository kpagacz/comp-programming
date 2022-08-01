// link to the problem: https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts/
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

// Runtime: 3 ms, faster than 77.54% of C++ online submissions for Make Array Zero by Subtracting Equal Amounts.
// Memory Usage: 8.3 MB, less than 50.84% of C++ online submissions for Make Array Zero by Subtracting Equal Amounts.

class Solution {
 public:
  int minimumOperations(std::vector<int>& nums) {
    std::sort(nums.begin(), nums.end());
    const auto& endUnique = std::unique(nums.begin(), nums.end());
    int distance = std::distance(nums.begin(), endUnique);
    return nums[0] == 0 ? distance - 1 : distance;
  }
};

int main(int argc, char** argv) {}
