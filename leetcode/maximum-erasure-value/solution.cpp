// link to the problem: https://leetcode.com/problems/maximum-erasure-value/
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

// I had a bug where the left pointer was not moved to the correct position after encountering a repetition.
// It turned that std::unordered_map::assign does not overwrite the value, only inserts it if it is not assigned.
// I needed to use insert_or_assign and it fixed the bug.

class Solution {
 public:
  int maximumUniqueSubarray(std::vector<int>& nums) {
    int maximum_score = 0;
    std::unordered_map<int, std::vector<int>::iterator> positions;
    int current_sum = 0;

    for (auto left = nums.begin(), right = nums.begin(); right != nums.end(); right++) {
      if (positions.find(*right) != positions.end()) {
        auto found = positions.at(*right);
        if (found >= left) {
          current_sum -= std::accumulate(left, found + 1, 0);
          left = found;
          left++;
        }
      }
      current_sum += *right;
      maximum_score = std::max(maximum_score, current_sum);
      positions.insert_or_assign(*right, right);
    }
    return maximum_score;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> v{5, 2, 1, 2, 5, 2, 1, 2, 5};
  s.maximumUniqueSubarray(v);
}
