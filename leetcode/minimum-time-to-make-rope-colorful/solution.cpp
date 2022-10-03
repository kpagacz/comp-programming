// link to the problem: https://leetcode.com/problems/minimum-time-to-make-rope-colorful/
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

// Runtime: 170 ms, faster than 90.41% of C++ online submissions for Minimum Time to Make Rope Colorful.
// Memory Usage: 95.5 MB, less than 50.05% of C++ online submissions for Minimum Time to Make Rope Colorful.

class Solution {
 public:
  int minCost(std::string colors, std::vector<int>& neededTime) {
    int answer = 0;
    int it = 0;
    while (it < colors.size()) {
      int secondIt = it + 1;
      int maxCost = neededTime[it];
      answer += neededTime[it];
      while (secondIt < colors.size() && colors[secondIt] == colors[it]) {
        answer += neededTime[secondIt];
        maxCost = std::max(maxCost, neededTime[secondIt]);
        secondIt++;
      }
      answer -= maxCost;
      it = secondIt;
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
