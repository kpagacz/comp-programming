// link to the problem: https://leetcode.com/problems/combination-sum-iv/
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
  int combinationSum4(std::vector<int>& nums, int target) {
    std::vector<uint64_t> combinations(target + 1001, 0);
    combinations[0] = 1;
    for (int i = 0; i < target; ++i) {
      for (const auto& num : nums) if (i + num <= target) combinations[i + num] += combinations[i];
    }
    return combinations[target];
  }
};

int main(int argc, char** argv) {
  std::vector<int> nums{1, 2, 3};
  int target = 32;
  Solution s;
  std::cout << s.combinationSum4(nums, target);
}
