// link to the problem: https://leetcode.com/problems/number-of-zero-filled-subarrays/
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

// Runtime: 308 ms, faster than 36.61% of C++ online submissions for Number of Zero-Filled Subarrays.
// Memory Usage: 107.6 MB, less than 46.72% of C++ online submissions for Number of Zero-Filled Subarrays.

class Solution {
 public:
  long long zeroFilledSubarray(std::vector<int>& nums) {
    int64_t answer = 0;
    auto it = nums.begin();
    while (it != nums.end()) {
      if (*it == 0) {
        auto forwardIt = it;
        while (forwardIt != nums.end() && *(forwardIt) == 0) ++forwardIt;
        int zerosCount = forwardIt - it;
        answer += ((int64_t)zerosCount * (zerosCount + 1)) / 2;
        it = forwardIt;
      } else {
        ++it;
      }
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  std::vector<int> nums{0, 2, 0};
  Solution s;
  std::cout << s.zeroFilledSubarray(nums);
}
