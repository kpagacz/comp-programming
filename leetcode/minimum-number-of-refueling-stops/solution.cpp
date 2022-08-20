// link to the problem: https://leetcode.com/problems/minimum-number-of-refueling-stops/
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

// Runtime: 28 ms, faster than 93.50% of C++ online submissions for Minimum Number of Refueling Stops.
// Memory Usage: 16.3 MB, less than 51.25% of C++ online submissions for Minimum Number of Refueling Stops.

class Solution {
 public:
  int minRefuelStops(int target, int startFuel, std::vector<std::vector<int>>& stations) {
    stations.push_back({target, 0});
    std::priority_queue<int> unvisitedStations;
    int stops = 0;
    for (const auto& station : stations) {
      while (startFuel < station[0] && !unvisitedStations.empty()) {
        startFuel += unvisitedStations.top();
        unvisitedStations.pop();
        stops++;
      }
      if (startFuel < station[0]) return -1;
      unvisitedStations.push(station[1]);
    }

    return stops;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::tuple<int, int, std::vector<std::vector<int>>>> cases{
      {1, 1, {}}, {100, 1, {{10, 100}}}, {100, 10, {{10, 60}, {20, 30}, {30, 30}, {60, 40}}}};

  for (auto& [target, startFuel, stations] : cases) {
    std::cout << s.minRefuelStops(target, startFuel, stations) << "\n";
  }
}
