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
    std::priority_queue<int, std::vector<int>> bricksMaxHeap;
    for (int i{0}; i < heights.size() - 1; i++) {
      const auto diff = heights[i + 1] - heights[i];
      if (diff <= 0) continue;
      if (bricks >= diff) {
        bricks -= diff;
        bricksMaxHeap.push(diff);
        continue;
      }

      if (ladders > 0) {
        --ladders;
        laddersMinHeap.push(diff);
        continue;
      }

      while(!laddersMinHeap.empty() && !bricksMaxHeap.empty() && laddersMinHeap.top() < bricksMaxHeap.top()) {
        int laddersTop = laddersMinHeap.top();
        int bricksTop = bricksMaxHeap.top();
        laddersMinHeap.pop();
        bricksMaxHeap.pop();
        laddersMinHeap.push(bricksTop);
        bricksMaxHeap.push(laddersTop);
        bricks += bricksTop - laddersTop;
      }

      if (bricks >= diff) {
        bricks -= diff;
        bricksMaxHeap.push(diff);
        continue;
      }

      while(!laddersMinHeap.empty() && bricks > laddersMinHeap.top()) {
        ++ladders;
        bricks -= laddersMinHeap.top();
        laddersMinHeap.pop();
      }

      if (ladders > 0) {
        --ladders;
        continue;
      }

      return i;
    }

    return heights.size() - 1;
  }
};

int main(int argc, char** argv) {}
