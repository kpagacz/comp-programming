// link to the problem: https://leetcode.com/problems/furthest-building-you-can-reach/
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
  int furthestBuilding(std::vector<int>& heights, int bricks, int ladders) {
    std::priority_queue<int, std::vector<int>, std::greater<int>> laddersMinHeap;
    for (int i{0}; i < heights.size() - 1; i++) {
      const auto diff = heights[i + 1] - heights[i];
      if (diff <= 0) continue;
      if (laddersMinHeap.size() < ladders) {
        laddersMinHeap.push(diff);
        continue;
      }

      if (!laddersMinHeap.empty() && diff > laddersMinHeap.top()) {
        bricks -= laddersMinHeap.top();
        laddersMinHeap.pop();
        laddersMinHeap.push(diff);
      } else {
        bricks -= diff;
      }

      if (bricks < 0) return i;
    }
    return heights.size() - 1;
  }
};

int main(int argc, char** argv) {}
