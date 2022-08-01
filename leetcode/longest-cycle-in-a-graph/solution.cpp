// link to the problem: https://leetcode.com/problems/longest-cycle-in-a-graph/
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

// Runtime: 226 ms, faster than 94.12% of C++ online submissions for Longest Cycle in a Graph.
// Memory Usage: 90.1 MB, less than 94.97% of C++ online submissions for Longest Cycle in a Graph.

class Solution {
 public:
  int longestCycle(std::vector<int>& edges) {
    std::vector<int> sets(edges.size(), -1);
    int maxLoop = -1;
    int setsCount = -1;
    for (int i = 0; i < edges.size(); ++i) {
      if (sets[i] != -1) continue;
      ++setsCount;
      int currentIndex = i;
      // if sets[currentIndex] != -1 then the node has already been visited before
      while (edges[currentIndex] != -1 && sets[currentIndex] == -1) {
        sets[currentIndex] = setsCount;
        currentIndex = edges[currentIndex];
      }

      int distance = -1;
      if (edges[currentIndex] != -1 && sets[currentIndex] == setsCount) {
        // there is a loop starting from index i
        // and it has not been counted previously
        // because sets[currentIndex] == setsCount
        int newCurrentIndex = edges[currentIndex];
        distance = 1;
        while (newCurrentIndex != currentIndex) {
          newCurrentIndex = edges[newCurrentIndex];
          ++distance;
        }
      }

      maxLoop = std::max(maxLoop, distance);
    }

    return maxLoop;
  }
};

int main(int argc, char** argv) {}
