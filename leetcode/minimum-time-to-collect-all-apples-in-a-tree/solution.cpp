// link to the problem: https://leetcode.com/problems/minimum-time-to-collect-all-apples-in-a-tree/
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
  int minTime(int n, std::vector<std::vector<int>>& edges, std::vector<bool>& hasApple) {
    std::vector<std::vector<int>> betterEdges(n);
    for (const auto& edge : edges) betterEdges[edge[0]].push_back(edge[1]), betterEdges[edge[1]].push_back(edge[0]);
    return minTimeRec(n, betterEdges, hasApple);
  }
  int minTimeRec(const int& n, const std::vector<std::vector<int>>& edges, const std::vector<bool>& hasApple,
                 const int& currentEdge = 0, const int& previousEdge = -1) {
    auto childrenTime = std::transform_reduce(edges.at(currentEdge).begin(), edges.at(currentEdge).end(), (int)0,
                                              std::plus<int>(), [&](const auto& destination) {
                                                if (destination == previousEdge) return 0;
                                                auto descendantTime =
                                                    minTimeRec(n, edges, hasApple, destination, currentEdge);
                                                return descendantTime == 0 ? 0 : descendantTime + 1;
                                              });
    if (currentEdge == 0) return childrenTime;
    else if (childrenTime != 0) return childrenTime + 1;
    else if (hasApple[currentEdge]) return 1;
    else return 0;
  };
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::tuple<int, std::vector<std::vector<int>>, std::vector<bool>>> cases;
  cases.push_back({1, {}, {true}});
  cases.push_back({4, {{0, 2}, {0, 3}, {1, 2}}, {false, true, false, false}});

  for (auto [n, edges, hasApple] : cases) std::cout << s.minTime(n, edges, hasApple) << '\n';
}
