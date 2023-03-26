// link to the problem: https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
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

// TLE
class Solution {
 public:
  long long countPairs(int n, std::vector<std::vector<int>>& edges) {
    std::vector<std::vector<bool>> reachablePairs(n, std::vector<bool>(n, false));
    for (const auto& edge : edges) {
      reachablePairs[edge[0]][edge[1]] = true;
      reachablePairs[edge[1]][edge[0]] = true;
    }

      for (std::size_t x = 0; x < reachablePairs.size(); x++)
        for (std::size_t y = 0; y < reachablePairs.size(); y++)
          for (std::size_t middleman = 0; middleman < reachablePairs.size(); middleman++)
            reachablePairs[x][y] =
                reachablePairs[x][y] || (reachablePairs[x][middleman] && reachablePairs[middleman][y]);

      int answer = 0;
      for (std::size_t start = 0; start < reachablePairs.size(); start++)
        for (std::size_t end = start + 1; end < reachablePairs.size(); end++)
          if (reachablePairs[start][end] == false) answer++;

      return answer;
  };
};

int main(int argc, char** argv) {}
