// link to the problem: https://leetcode.com/problems/next-permutation/
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
  void nextPermutation(std::vector<int>& nums) {
    auto rsorted_end = std::is_sorted_until(nums.rbegin(), nums.rend());
    bool has_more_permutations = rsorted_end != nums.rend();
    if (has_more_permutations) {
      auto rupper_bound = std::upper_bound(nums.rbegin(), rsorted_end, *rsorted_end);
      std::iter_swap(rsorted_end, rupper_bound);
    }

    std::reverse(nums.rbegin(), rsorted_end);
  }
};

int main(int argc, char** argv) {}
