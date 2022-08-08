// link to the problem: https://leetcode.com/problems/reachable-nodes-with-restrictions/
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

// Runtime: 568 ms, faster than 55.56% of C++ online submissions for Reachable Nodes With Restrictions.
// Memory Usage: 143.7 MB, less than 33.33% of C++ online submissions for Reachable Nodes With Restrictions.

class Solution {
 public:
  int reachableNodes(int n, std::vector<std::vector<int>>& edges, std::vector<int>& restricted) {
    std::vector<std::vector<int>> betterEdges(n, std::vector<int>());
    std::unordered_set<int> betterRestricted(restricted.begin(), restricted.end());
    for (const auto& edge : edges) {
      if (betterRestricted.count(edge[0]) || betterRestricted.count(edge[1])) continue;
      betterEdges[edge[0]].push_back(edge[1]);
      betterEdges[edge[1]].push_back(edge[0]);
    }
    edges.push_back({0, 0});

    int answer = 0;
    std::queue<int> toVisit;
    toVisit.push(0);

    while (!toVisit.empty()) {
      auto top = toVisit.front();
      toVisit.pop();
      if (edges[top][0] == -1) continue;
      for (const auto& destination : betterEdges[top]) toVisit.push(destination);
      edges[top][0] = -1;
      ++answer;
    }

    return answer;
  }
};

int main(int argc, char** argv) {
  // std::vector<std::vector<int>> edges{{0, 1}, {1, 2}, {3, 1}, {4, 0}, {0, 5}, {5, 6}};
  // std::vector<int> restricted{4, 5};
  std::vector<std::vector<int>> edges{{0, 1}, {0, 2}, {0, 5}, {0, 4}, {3, 2}, {6, 5}};
  std::vector<int> restricted{4, 2, 1};
  int n = 7;

  Solution s;
  std::cout << s.reachableNodes(n, edges, restricted) << '\n';
}
