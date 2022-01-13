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

class Solution {
 public:
  int findMinArrowShots(std::vector<std::vector<int>>& points) {
    auto compare = [](const std::vector<int>& a, const std::vector<int>& b) { return a[1] < b[1]; };
    std::sort(points.begin(), points.end(), compare);
    int arrows{0}, last_arrow{INT_MIN};
    for(const auto& baloon : points) {
      if (arrows == 0 || baloon[0] > last_arrow) {
        arrows++;
        last_arrow = baloon[1];
      }
    }

    return arrows;
  }
};

int main(int argc, char** argv) {}
