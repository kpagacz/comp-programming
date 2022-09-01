// link to the problem: https://leetcode.com/problems/count-lattice-points-inside-a-circle/
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

// Runtime: 1271 ms, faster than 50.86% of C++ online submissions for Count Lattice Points Inside a Circle.
// Memory Usage: 26.2 MB, less than 5.24% of C++ online submissions for Count Lattice Points Inside a Circle.

class Solution {
 public:
  int countLatticePoints(std::vector<std::vector<int>>& circles) {
    auto hashFunction = [](const std::pair<int, int>& pair) { return (pair.second << 16) ^ pair.first; };
    auto comparator = [](const std::pair<int, int>& first, const std::pair<int, int>& second) {
      return first.first == second.first && first.second == second.second;
    };
    std::unordered_set<std::pair<int, int>, decltype(hashFunction), decltype(comparator)> points(100, hashFunction,
                                                                                                 comparator);

    for (const auto& circle : circles) {
      for (int x = circle[0] - circle[2]; x <= circle[0] + circle[2]; ++x)
        for (int y = circle[1] - circle[2]; y <= circle[1] + circle[2]; ++y)
          if (isOnCircle(x, y, circle)) points.insert({x, y});
    }
    return points.size();
  }

  bool isOnCircle(const int& x, const int& y, const std::vector<int>& circle) {
    const int dx = std::abs(x - circle[0]);
    const int dy = std::abs(y - circle[1]);
    if (dx > circle[2] || dy > circle[2]) return false;
    if (dx + dy <= circle[2]) return true;
    return dx * dx + dy * dy <= circle[2] * circle[2];
  }
};

int main(int argc, char** argv) {}
