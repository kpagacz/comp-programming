// link to the problem: https://leetcode.com/problems/paint-house-iii/
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
  int minCost(std::vector<int>& houses, std::vector<std::vector<int>>& cost, int m, int n, int target) {
    return 0;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> houses{0, 2, 1, 2, 0};
  std::vector<std::vector<int>> cost{{1, 10}, {10, 1}, {10, 1}, {1, 10}, {5, 1}};

  int m = 5;
  int n = 2;
  int target = 3;
  std::cout << s.minCost(houses, cost, m, n, target);
}
