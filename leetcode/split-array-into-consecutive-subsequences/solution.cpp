// link to the problem: https://leetcode.com/problems/split-array-into-consecutive-subsequences/
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

constexpr int SHIFT = 1000;

class Solution {
 public:
  bool isPossible(std::vector<int>& nums) {
    std::vector<int> counts(2001, 0);
    for (const auto& num : nums) counts[num + SHIFT]++;
    std::unordered_map<int, int> ends;
    for (const auto& num : nums) {
      if (counts[num + SHIFT] == 0) continue;
      counts[num + SHIFT]--;

      if (ends[num - 1] > 0) {
        ends[num - 1]--;
        ends[num]++;
      } else if (counts[num + 1 + SHIFT] > 0 && counts[num + 2 + SHIFT] > 0) {
        counts[num + 1 + SHIFT]--;
        counts[num + 2 + SHIFT]--;
        ends[num + 2]++;
      } else
        return false;
    }

    return true;
  }
};

int main(int argc, char** argv) {}
