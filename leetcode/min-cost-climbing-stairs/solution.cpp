// link to the problem: https://leetcode.com/problems/min-cost-climbing-stairs/
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
  int minCostClimbingStairs(std::vector<int>& cost) {
    std::vector<int> scores(cost.size() + 1);
    scores[0] = cost[0];
    scores[1] = cost[1];
    for (int i = 2; i < cost.size(); ++i) {
      scores[i] = cost[i];
      scores[i] += std::min(scores[i - 1], scores[i - 2]);
    }

    return std::min(scores[cost.size() - 1], scores[cost.size() - 2]);
  }
};

int main(int argc, char** argv) {}
