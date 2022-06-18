// link to the problem: https://leetcode.com/problems/climbing-stairs/
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
  int climbStairs(int n) {
    std::vector<int> ways(n + 1, 0);
    ways[0] = 1;
    for (auto i{0}; i + 2 <= n; i++) {
      ways[i + 1] += ways[i];
      ways[i + 2] += ways[i];
    }
    ways[n] += ways[n - 1];
    return ways[n];
  }
};

int main(int argc, char** argv) {}
