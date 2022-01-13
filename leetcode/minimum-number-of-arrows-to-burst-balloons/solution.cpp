// link to the problem: https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using point = std::array<int, 3>;  // x, ballon, start/end(0/1)

class Solution {
 public:
  int findMinArrowShots(std::vector<std::vector<int>>& points) {
    auto compare = [](const point& a, const point& b) {
      if (a[0] == b[0]) return a[2] > b[2];
      else return a[0] > b[0];
    };
    std::priority_queue<point, std::vector<point>, decltype(compare)> events(compare);
    for (auto i{0}; i < points.size(); i++) {
      events.push({points[i][0], i, 0});
      events.push({points[i][1], i, 1});
    }

    int arrows{0};
    std::vector<int> current_ballons;
    std::vector<bool> popped(points.size(), false);
    while (!events.empty()) {
      auto event = events.top();
      events.pop();
      if (event[2] == 1 && popped[event[1]]) continue;
      if (event[2] == 0) {
        current_ballons.push_back(event[1]);
        continue;
      }
      assert(event[2] == 1);
      for (const auto& a : current_ballons) popped[a] = true;
      current_ballons.clear();
      arrows++;
    }

    return arrows;
  }
};

int main(int argc, char** argv) {}
