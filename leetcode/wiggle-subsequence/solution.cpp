// link to the problem: https://leetcode.com/problems/wiggle-subsequence/
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

// Runtime: 3 ms, faster than 69.26% of C++ online submissions for Wiggle Subsequence.
// Memory Usage: 7.2 MB, less than 43.94% of C++ online submissions for Wiggle Subsequence.

class Solution {
 public:
  int wiggleMaxLength(std::vector<int>& nums) {
    std::pair<int, Next> state;  // last integer of the wiggle; whether the next integer should be larger or smaller
    int answer = 0;
    if (nums.size() <= 1) return nums.size();
    state.first = nums[0];
    ++answer;
    int i = 1;
    while (i < nums.size() && nums[i] == state.first) ++i;
    if (i == nums.size()) return answer;

    // std::cout << "nums[i]: " << nums[i] << " state.first: " << state.first << '\n';
    if (nums[i] > state.first)  // 1 8
      state.second = Next::DOWN;
    else  // 8 1
      state.second = Next::UP;

    state.first = nums[i];
    ++answer;
    // std::cout << "Length: " << answer << " state last: " << state.first << " next: " << state.second << "\n";

    while (++i < nums.size()) {
      if (state.second == Next::UP) {
        if (nums[i] > state.first) {
          ++answer;
          state.second = Next::DOWN;
        }
      } else {
        if (nums[i] < state.first) {
          ++answer;
          state.second = Next::UP;
        }
      }
      state.first = nums[i];
      // std::cout << "Length: " << answer << " state last: " << state.first << " next: " << state.second << "\n";
    }

    return answer;
  }

 private:
  enum Next { UP, DOWN };
};
