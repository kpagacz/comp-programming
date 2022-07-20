// link to the problem: https://leetcode.com/problems/longest-consecutive-sequence/
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

// Runtime: 139 ms, faster than 80.39% of C++ online submissions for Longest Consecutive Sequence.
// Memory Usage: 75.5 MB, less than 5.02% of C++ online submissions for Longest Consecutive Sequence.

class Solution {
 public:
  int longestConsecutive(std::vector<int>& nums) {
    if (nums.size() == 0) return 0;
    std::vector<std::vector<int>> buckets(2 * 1e9 / 1e5 + 1, std::vector<int>());
    for (const auto& num : nums) {
      const auto bucket = (num + 1e9) / 1e5;
      buckets[bucket].push_back(num);
    }

    std::vector<int> sorted;
    sorted.reserve(nums.size());
    for (auto& bucket : buckets) {
      std::sort(bucket.begin(), bucket.end());
      std::copy(bucket.begin(), bucket.end(), std::back_inserter(sorted));
    }
    int longestSequence = 1, currentSequence = 1;
    int previousNum = sorted[0];
    for (const auto& num : sorted) {
      if (num == previousNum) continue;
      if (num - previousNum == 1) {
        ++currentSequence;
        longestSequence = std::max(longestSequence, currentSequence);
      } else {
        currentSequence = 1;
      }
      previousNum = num;
    }
    return longestSequence;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> v{100, 4, 200, 1, 3, 2};
  std::cout << s.longestConsecutive(v);
}
