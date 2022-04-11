// link to the problem: https://leetcode.com/problems/shortest-path-visiting-all-nodes/
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
  int shortestPathLength(std::vector<std::vector<int>>& graph) {
    std::queue<std::tuple<int, int, int>> bfs_queue;  // current node, visited nodes bitmask, cost
    std::set<std::pair<int, int>> seen_states;        // current node, visited nodes bitmask
    for (auto i{0}; i < graph.size(); i++) {
      bfs_queue.push({i, 1 << i, 0});
      seen_states.insert({i, 1 << i});
    }

    while (!bfs_queue.empty()) {
      auto front = bfs_queue.front();
      bfs_queue.pop();

      if (std::get<1>(front) == (1 << graph.size()) - 1) return std::get<2>(front);

      for (const auto& neighbour : graph[std::get<0>(front)]) {
        auto next_step =
            std::make_tuple(neighbour, std::get<1>(front) | (1 << neighbour), std::get<2>(front) + 1);
        if (seen_states.find({std::get<0>(next_step), std::get<1>(next_step)}) == seen_states.end()) {
          bfs_queue.push(next_step);
          seen_states.insert({std::get<0>(next_step), std::get<1>(next_step)});
        }
      }
    }

    return -1;
  }
};

int main(int argc, char** argv) {}
