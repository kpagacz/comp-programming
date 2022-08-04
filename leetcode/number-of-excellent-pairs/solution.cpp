// link to the problem: https://leetcode.com/problems/number-of-excellent-pairs/
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

// Runtime: 237 ms, faster than 98.16% of C++ online submissions for Number of Excellent Pairs.
// Memory Usage: 59.5 MB, less than 98.20% of C++ online submissions for Number of Excellent Pairs.

class Solution {
 public:
  long long countExcellentPairs(std::vector<int>& nums, int k) {
    std::sort(nums.begin(), nums.end());
    nums.erase(std::unique(nums.begin(), nums.end()), nums.end());
    int64_t answer = 0;
    for (auto& num : nums) {
      int setBits = 0;
      while (num) {
        ++setBits;
        num = num & (num - 1);
      }
      num = setBits;
    }

    std::sort(nums.begin(), nums.end());

    int left = 0, right = nums.size() - 1;

    // std::copy(nums.begin(), nums.end(), std::ostream_iterator<int>(std::cout, " "));
    // std::cout << '\n';

    while (left <= right) {
      if (nums[left] + nums[right] >= k) {
        answer += (right - left) * 2 + 1;
        --right;
      } else {
        ++left;
      }
    }

    return answer;
  }
};

int main(int argc, char** argv) {
  std::vector<int> v{1, 2, 3, 1};
  Solution s;
  std::cout << s.countExcellentPairs(v, 3);
}
