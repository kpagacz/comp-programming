// link to the problem: https://leetcode.com/problems/jump-game-vi/
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

// This one gets TLE

class Solution {
 public:
  int maxResult(std::vector<int>& nums, int k) {
    std::vector<int> maxScores(nums.size(), INT_MIN);
    maxScores[0] = nums[0];
    for (int i = 0; i < maxScores.size(); ++i) {
      int currentScore = maxScores[i];
      for (int j = i + 1; j < maxScores.size() && j <= i + k; ++j) {
        maxScores[j] = std::max(maxScores[j], currentScore + nums[j]);
      }
    }
    return maxScores[nums.size() - 1];
  }
};

int main(int argc, char** argv) {}
