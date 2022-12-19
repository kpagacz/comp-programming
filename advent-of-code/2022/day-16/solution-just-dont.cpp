#include <algorithm>
#include <cassert>
#include <cinttypes>
#include <cstddef>
#include <fstream>
#include <functional>
#include <iostream>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "fmt/fmt/core.h"

class Solution {
 public:
  int part1(const std::string& path) const {
    auto [ids, names, pressures, destinations] = parseInput(path);
    assert(ids.size() <= 64);
    std::cout << '\n';
    for (const auto& name : names) {
      fmt::print("Valve:{} pressure:{} destinations:", name, pressures.at(ids.at(name)));
      for (const auto& d : destinations.at(ids.at(name))) fmt::print("{} ", names[d]);
      fmt::print("\n");
    }
    int time = 0;
    std::vector<uint8_t> pressuredValves;
    int totalPressure = 0;
    for (const auto& [valve, pressure] : pressures)
      if (pressure > 0) pressuredValves.push_back(valve), totalPressure += pressure;

    fmt::print("Pressured valves: ");
    for (const auto& v : pressuredValves) fmt::print("{} ", v);
    fmt::print("\n");

    std::vector<std::vector<int>> shortestPaths, predecessors;
    for (int i = 0; i < names.size(); i++) {
      auto [paths, preds] = bfs(i, destinations);
      shortestPaths.push_back(paths), predecessors.push_back(preds);
    }
    fmt::print("Shortest paths:\n");
    for (const auto& name : names) fmt::print("{} ", name);
    fmt::print("\n");
    for (const auto& lengths : shortestPaths) {
      for (const auto& length : lengths) fmt::print("{} ", length);
      fmt::print("\n");
    }

    using state = std::tuple<uint64_t, uint8_t, int, int, int>;
    std::queue<state> states;  // opened, current, flow, released, timeUsed
    states.push({0ull, ids.at("AA"), 0, 0, 0});

    // auto optimizeDestinations = [&](const uint8_t& source, const int& timeLeft) {
    //   auto destinations = pressuredValves;
    //   std::vector<bool> toTake(destinations.size(), true);
    //   for (int i = 0; i < destinations.size(); i++)
    //     for (int j = i + 1; j < destinations.size(); i++) {
    //       auto timeToFirst = shortestPaths[source][pressuredValves[i]] + 1;
    //       auto timeToSecond = shortestPaths[source][pressuredValves[j]] + 1;
    //       if (timeToFirst > timeLeft) toTake[i] = false;
    //       if (timeToSecond > timeLeft) toTake[j] = false;
    //       auto releasedWithFirst = (timeLeft - timeToFirst) * pressures[pressuredValves[i]];
    //     }
    // };
    const int TIME_LIMIT = 30;
    int maxReleased = 0;
    while (!states.empty()) {
      auto [openedValves, source, flow, released, timeLeft] = states.front();
      states.pop();

      maxReleased = std::max(maxReleased, released + flow * timeLeft);
      if (timeLeft == 0) continue;
    }

    return maxReleased;
  }

 private:
  bool isClosed(const uint8_t& valve, const uint64_t& opened) const {
    return !(opened & (1ull << valve));
  }
  bool isAddedPreviously(const uint8_t& valve, const uint64_t& addedPreviously) const {
    return addedPreviously & (1ull << valve);
  }
  const std::string VALVE_NAME_PREFIX = "Valve ";
  const std::string FLOW_RATE_PREFIX = "rate=";
  std::tuple<std::unordered_map<std::string, int>, std::vector<std::string>, std::unordered_map<uint8_t, int>,
             std::unordered_map<uint8_t, std::vector<uint8_t>>>
  parseInput(const std::string& path) const {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    std::unordered_map<std::string, int> ids;
    std::vector<std::string> names;
    std::unordered_map<uint8_t, int> pressures;
    std::unordered_map<uint8_t, std::vector<uint8_t>> paths;
    while (std::getline(input, line)) {
      std::stringstream ss(line);
      std::string valveName;
      for (int i = 0; i < 2; i++) ss >> valveName;
      if (!ids.contains(valveName)) {
        ids.insert({valveName, ids.size()});
        names.push_back(valveName);
      }
      assert(ids.size() == names.size());
      assert(names[ids[valveName]] == valveName);

      std::string flowRateToken;
      for (int i = 0; i < 3; i++) ss >> flowRateToken;
      auto flowRate = std::stoi(flowRateToken.substr(5, flowRateToken.find(";", 5)));
      pressures.insert({ids[valveName], flowRate});

      std::string valveDestination;
      for (int i = 0; i < 4; i++) ss >> valveDestination;
      while (ss >> valveDestination) {
        auto comma = valveDestination.find(",");
        if (comma != valveDestination.npos) valveDestination.erase(comma, 1);
        if (!ids.contains(valveDestination)) {
          ids.insert({valveDestination, ids.size()});
          names.push_back(valveDestination);
        }
        assert(ids.size() == names.size());
        assert(names[ids[valveName]] == valveName);
        paths[ids[valveName]].push_back(ids[valveDestination]);
      }
    }
    return {ids, names, pressures, paths};
  }

  std::pair<std::vector<int>, std::vector<int>> bfs(
      const uint8_t& source, const std::unordered_map<uint8_t, std::vector<uint8_t>>& paths) const {
    std::vector<int> shortestPath(paths.size()), predecessor(paths.size());
    shortestPath[source] = 0, predecessor[source] = source;
    std::queue<std::tuple<uint8_t, int>> bfs;  // visited, current node, length
    bfs.push({source, 0});
    uint64_t visited = 1ull << source;
    while (!bfs.empty()) {
      auto [id, length] = bfs.front();
      bfs.pop();
      for (const auto& destination : paths.at(id)) {
        if ((visited >> destination) % 2 == 0) {
          shortestPath[destination] = length + 1, predecessor[destination] = id;
          bfs.push({destination, length + 1});
        }
      }
      visited |= (1ull << id);
    }
    return {shortestPath, predecessor};
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("test") << '\n';
  for (uint64_t i = 0; i < 1e12; i++) {
  }
  return 0;
}
