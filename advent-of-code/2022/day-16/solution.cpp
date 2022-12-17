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
    auto [ids, pressures, destinations] = parseInput(path);
    assert(ids.size() <= 64);
    std::vector<std::string> names(ids.size());
    for (const auto& [valve, id] : ids) names[id] = valve;
    for (const auto& [valve, id] : ids) {
      fmt::print("Valve:{} id:{} p:{} ds:", valve, id, pressures.at(valve));
      for (const auto& d : destinations.at(valve)) fmt::print("{} ", d);
      fmt::print("\n");
    }

    // for (int i = 0; i < shortest.size(); i++) {
    //   fmt::print("Target:{} l:{} pred:{}\n", names[i], shortest[i], predecessor[i]);
    // }
    // fmt::print("\n");

    std::vector<std::vector<int>> shortestPaths(ids.size()), predecessors(ids.size());
    for (const auto& [valve, id] : ids) {
      auto [lengths, preds] = bfs(valve, ids, destinations);
      shortestPaths[id] = lengths;
      predecessors[id] = preds;
    }

    std::vector<int> valves;
    for (const auto& [valve, press] : pressures)
      if (press != 0) valves.push_back(ids.at(valve));

    std::sort(valves.begin(), valves.end());
    int maxReleasedPressure = 0;
    while (std::next_permutation(valves.begin(), valves.end())) {
      int time = 30;
      int releasedPressure = 0;
      int flowRate = 0;
      int source = ids.at("AA");
      for (const auto& dest : valves) {
        if (time <= 0) break;
        int length = shortestPaths[source][dest];
        int realized = std::min(length, time);
        releasedPressure += realized * flowRate;
        time -= realized;
        if (time <= 0) break;
        releasedPressure += flowRate;
        flowRate += pressures.at(names[dest]);
        time--;
        source = dest;
      }
      if (time > 0) releasedPressure += flowRate * time;
      maxReleasedPressure = std::max(maxReleasedPressure, releasedPressure);
    }

    return maxReleasedPressure;
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

  std::pair<std::vector<int>, std::vector<int>> bfs(
      const std::string& source, const std::unordered_map<std::string, int>& labels,
      const std::unordered_map<std::string, std::vector<std::string>>& paths) {
    int sourceId = labels.at(source);
    std::vector<int> shortestPath(labels.size()), predecessor(labels.size());
    shortestPath[sourceId] = 0, predecessor[sourceId] = sourceId;
    std::queue<std::tuple<std::string, int>> bfs;  // visited, current node, length
    bfs.push({source, 0});
    uint64_t visited = 1ull << sourceId;
    while (!bfs.empty()) {
      auto [label, length] = bfs.front();
      bfs.pop();
      for (const auto& destination : paths.at(label)) {
        int dest = labels.at(destination);
        if ((visited >> dest) % 2 == 0) {
          shortestPath[dest] = length + 1, predecessor[dest] = labels.at(label);
          bfs.push({destination, length + 1});
        }
      }
      visited |= (1ull << labels.at(label));
    }
    return {shortestPath, predecessor};
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  return 0;
}
