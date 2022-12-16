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

class hash_tuple {
  template <class T>
  struct component {
    const T& value;
    component(const T& value) : value(value) {}
    uintmax_t operator,(uintmax_t n) const {
      n ^= std::hash<T>()(value);
      n ^= n << (sizeof(uintmax_t) * 4 - 1);
      return n ^ std::hash<uintmax_t>()(n);
    }
  };

 public:
  template <class Tuple>
  size_t operator()(const Tuple& tuple) const {
    return std::hash<uintmax_t>()(std::apply([](const auto&... xs) { return (component(xs), ..., 0); }, tuple));
  }
};

class Solution {
 public:
  int part1(const std::string& path) {
    auto [labels, pressures, paths] = parseInput(path);
    assert(labels.size() <= 64);
    std::unordered_map<std::string, int> flowPotentials;

    std::queue<std::tuple<uint64_t, std::string, int, int, int>>
        bfs;  // openedValves, valve, flowRate, released, timeLeft
    bfs.push({1ull << labels["AA"], "AA", 0, 0, 30});
    int maxReleasePressure = 0;
    while (!bfs.empty()) {
      // std::cout << "Size of the queue: " << bfs.size() << '\n';
      // if (bfs.size() > 10000) break;
      auto [openedValves, valve, flowRate, released, timeLeft] = bfs.front();
      bfs.pop();

      // fmt::print("Current: {:b} valve:{} flowRate:{} released:{} timeLeft:{}\n", openedValves, valve, flowRate,
      //  released, timeLeft);
      // Stop if time ran out
      if (timeLeft == 0) {
        maxReleasePressure = std::max(maxReleasePressure, released);
        continue;
      }
      // If this node was already visited with the same set of the open valves
      // but lower flowPotential, no point continuing this branch
      auto flowPotential = released + flowRate * timeLeft;
      bool condition = flowPotentials.contains(valve) && flowPotentials.at(valve) > flowPotential;
      // fmt::print("Current flow potential {} condition: {}\n", flowPotential, condition);
      if (flowPotentials.contains(valve) && flowPotentials.at(valve) > flowPotential) {
        // fmt::print("lower potential\n");
        continue;
      }
      // If this node was already visisted but we did not improve potential,
      // just project the final potential, because we are in a loop
      if (flowPotentials.contains(valve) && flowPotentials.at(valve) == flowPotential) {
        maxReleasePressure = std::max(maxReleasePressure, flowPotential);
        // fmt::print("Loop\n");
        continue;
      }

      // open the valve if its flowRate is 0
      if (pressures.at(valve) == 0) openedValves = (1 << labels[valve]) | openedValves;

      released += flowRate;
      timeLeft--;

      for (const auto& destination : paths.at(valve)) {
        bfs.push({openedValves, destination, flowRate, released, timeLeft});
      }

      assert(flowPotentials.contains(valve) == false || flowPotential > flowPotentials.at(valve));
      flowPotentials[valve] = released + timeLeft * flowRate;
      // open the valve if not opened
      // fmt::print("label: {}\n", labels.at(valve));
      // fmt::print("openedValves: {:b} ", openedValves);
      // fmt::print("{:b} condition: {}\n", openedValves >> labels.at(valve), (openedValves >> labels.at(valve)) % 2 ==
      // 0);
      if ((openedValves >> labels.at(valve)) % 2 == 0) {
        openedValves |= 1 << labels.at(valve);
        flowRate += pressures.at(valve);
        bfs.push({openedValves, valve, flowRate, released, timeLeft});
      }
    }
    return maxReleasePressure;
  }

 private:
  const std::string VALVE_NAME_PREFIX = "Valve ";
  const std::string FLOW_RATE_PREFIX = "rate=";
  std::tuple<std::unordered_map<std::string, int>, std::unordered_map<std::string, int>,
             std::unordered_map<std::string, std::vector<std::string>>>
  parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    std::unordered_map<std::string, int> labels;
    std::unordered_map<std::string, int> pressures;
    std::unordered_map<std::string, std::vector<std::string>> paths;
    while (std::getline(input, line)) {
      std::stringstream ss(line);
      std::string valveName;
      for (int i = 0; i < 2; i++) ss >> valveName;
      if (!labels.contains(valveName)) labels.insert({valveName, labels.size()});

      std::string flowRateToken;
      for (int i = 0; i < 3; i++) ss >> flowRateToken;
      auto flowRate = std::stoi(flowRateToken.substr(5, flowRateToken.find(";", 5)));
      pressures.insert({valveName, flowRate});

      std::string valveDestination;
      for (int i = 0; i < 4; i++) ss >> valveDestination;
      while (ss >> valveDestination) {
        auto comma = valveDestination.find(",");
        if (comma != valveDestination.npos) valveDestination.erase(comma, 1);
        paths[valveName].push_back(valveDestination);
      }
    }

    return {labels, pressures, paths};
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  return 0;
}
