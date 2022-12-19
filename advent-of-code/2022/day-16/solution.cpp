#include <algorithm>
#include <bit>
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

using State = std::tuple<uint16_t, uint8_t, int>;
struct TupleHash {
  std::size_t operator()(const State& state) const {
    std::size_t hash{0};
    hash = std::hash<uint16_t>()(std::get<0>(state));
    hash ^= std::hash<uint8_t>()(std::get<1>(state)) << 10;
    hash ^= std::hash<int>()(std::get<2>(state)) << 20;
    return hash;
  }
};

using State2 = std::tuple<uint16_t, uint8_t, uint8_t, int, int, int>;
struct State2Hash {
  std::size_t operator()(const State2& state) const {
    std::size_t hash{0};
    hash = std::hash<uint16_t>()(std::get<0>(state));
    hash ^= std::hash<uint8_t>()(std::get<1>(state)) << 10;
    hash ^= std::hash<uint8_t>()(std::get<2>(state)) << 20;
    hash ^= std::hash<int>()(std::get<3>(state)) << 5;
    hash ^= std::hash<int>()(std::get<4>(state)) << 7;
    hash ^= std::hash<int>()(std::get<5>(state)) << 16;
    return hash;
  }
};

class Solution {
 public:
  int part1(const std::string& path) const {
    auto [ids, names, pressures, destinations] = parseInput(path);
    assert(ids.size() <= 64);

    std::vector<uint8_t> pressuredValves;
    for (const auto& [valve, pressure] : pressures)
      if (pressure > 0) pressuredValves.push_back(valve);
    pressuredValves.push_back(ids.at("AA"));

    std::vector<std::vector<int>> shortestPaths, predecessors;
    for (int i = 0; i < names.size(); i++) {
      auto [paths, preds] = bfs(i, destinations);
      shortestPaths.push_back(paths), predecessors.push_back(preds);
    }

    assert(pressuredValves.size() <= 16);
    uint16_t openedValves = 0;
    short start = pressuredValves.size() - 1;
    openedValves |= 1u << start;
    assert(!isClosed(pressuredValves.size() - 1, openedValves));
    std::unordered_map<State, int, TupleHash> cache;  // opened valves, index in pressuredValves (current), timeLeft

    auto releasedPressure = [&, &pressures = pressures](auto& self, const State& state) {
      if (cache.contains(state)) return cache.at(state);
      const auto& [opened, current, timeLeft] = state;
      if (timeLeft < 2) return 0;
      const auto& currentValveId = pressuredValves[current];
      int pressureReleasedFromCurrent = (timeLeft - 1) * pressures.at(currentValveId);
      auto newTime = current == pressuredValves.size() - 1 ? timeLeft : timeLeft - 1;
      auto newOpened = opened | 1u << current;
      auto releasedByOthers = 0;
      for (int destination = 0; destination < pressuredValves.size(); destination++)
        if (isClosed(destination, newOpened))
          releasedByOthers = std::max(
              releasedByOthers, self(self, {newOpened, destination,
                                            newTime - shortestPaths[currentValveId][pressuredValves[destination]]}));
      cache[state] = pressureReleasedFromCurrent + releasedByOthers;
      return pressureReleasedFromCurrent + releasedByOthers;
    };

    return releasedPressure(releasedPressure, {openedValves, start, 30});
  }

  int part2(const std::string& path) const {
    auto [ids, names, pressures, destinations] = parseInput(path);
    assert(ids.size() <= 64);

    std::vector<uint8_t> pressuredValves;
    for (const auto& [valve, pressure] : pressures)
      if (pressure > 0) pressuredValves.push_back(valve);
    pressuredValves.push_back(ids.at("AA"));

    std::vector<std::vector<int>> shortestPaths, predecessors;
    for (int i = 0; i < names.size(); i++) {
      auto [paths, preds] = bfs(i, destinations);
      shortestPaths.push_back(paths), predecessors.push_back(preds);
    }
    for (auto& r : shortestPaths)
      for (auto& v : r) v++;

    // Debug printing
    // fmt::print("Pressured valves: ");
    // for (const auto& v : pressuredValves) fmt::print("{} ", v);
    // fmt::print("\n");

    // fmt::print("pressures:\n");
    // for (int i = 0; i < pressuredValves.size(); i++) {
    //   auto valve = pressuredValves[i];
    //   fmt::print("{} pos:{} {} p:{}\n", valve, i, names.at(valve), pressures.at(valve));
    // }

    // for (const auto& r : shortestPaths) {
    //   for (const auto& d : r) fmt::print("{} ", d);
    //   fmt::print("\n");
    // }

    assert(pressuredValves.size() <= 16);
    uint16_t openedValves = 0;
    short start = pressuredValves.size() - 1;
    openedValves |= 1u << start;
    assert(!isClosed(pressuredValves.size() - 1, openedValves));
    std::unordered_map<State2, int, State2Hash> cache;  // state -> releasedPressure

    // Recursive function to calculate released pressure
    auto releasedPressure = [&, &pressures = pressures, &names = names](auto& self, const State2& state) {
      if (cache.contains(state)) return cache.at(state);
      const auto& [opened, meCurrent, elephantCurrent, meEta, elephantEta, timeLeft] = state;

      if (timeLeft < 1) return 0;
      assert(meEta == 0 || elephantEta == 0);

      // Add pressure from current valves
      int pressureReleasedFromCurrent = 0, releasedByOthers = 0;
      if (meEta == 0 && isClosed(meCurrent, opened))
        pressureReleasedFromCurrent += (timeLeft)*pressures.at(pressuredValves.at(meCurrent));
      if (elephantEta == 0 && isClosed(elephantCurrent, opened))
        pressureReleasedFromCurrent += (timeLeft)*pressures.at(pressuredValves.at(elephantCurrent));

      // Add pressure from others
      if (meEta == 0 && elephantEta == 0) {
        auto newOpened = opened | 1u << meCurrent;
        newOpened |= 1u << elephantCurrent;
        for (int meDestination = 0; meDestination < pressuredValves.size(); meDestination++)
          for (int elephantDestination = 0; elephantDestination < pressuredValves.size(); elephantDestination++) {
            if (meDestination == elephantDestination) continue;
            if (isClosed(meDestination, newOpened) && isClosed(elephantDestination, newOpened)) {
              auto newElephantEta =
                  shortestPaths[pressuredValves.at(elephantCurrent)][pressuredValves.at(elephantDestination)];
              auto newMeEta = shortestPaths[pressuredValves.at(meCurrent)][pressuredValves.at(meDestination)];
              auto timeDecrement = std::min(newElephantEta, newMeEta);
              releasedByOthers = std::max(
                  releasedByOthers, self(self, {newOpened, meDestination, elephantDestination, newMeEta - timeDecrement,
                                                newElephantEta - timeDecrement, timeLeft - timeDecrement}));
            }
          }
      } else if (meEta == 0) {
        auto newOpened = opened | (1u << meCurrent);
        if (std::popcount(newOpened) == pressuredValves.size() - 1) {
          releasedByOthers = std::max(
              releasedByOthers, self(self, {newOpened, meCurrent, elephantCurrent, 0, 0, timeLeft - elephantEta}));
        } else {
          for (int destination = 0; destination < pressuredValves.size(); destination++) {
            if (!isClosed(destination, newOpened) || destination == elephantCurrent) continue;
            auto distanceToDestination = shortestPaths[pressuredValves.at(meCurrent)][pressuredValves.at(destination)];
            auto timeDecrement = std::min(distanceToDestination, elephantEta);
            releasedByOthers =
                std::max(releasedByOthers,
                         self(self, {newOpened, destination, elephantCurrent, distanceToDestination - timeDecrement,
                                     elephantEta - timeDecrement, timeLeft - timeDecrement}));
          }
        }
      } else {
        auto newOpened = opened | (1u << elephantCurrent);
        if (std::popcount(newOpened) == pressuredValves.size() - 1) {
          releasedByOthers =
              std::max(releasedByOthers, self(self, {newOpened, meCurrent, elephantCurrent, 0, 0, timeLeft - meEta}));
        } else {
          for (int destination = 0; destination < pressuredValves.size(); destination++) {
            if (!isClosed(destination, newOpened) || destination == meCurrent) continue;
            auto distanceToDestination =
                shortestPaths[pressuredValves.at(elephantCurrent)][pressuredValves.at(destination)];
            auto timeDecrement = std::min(distanceToDestination, meEta);
            releasedByOthers = std::max(releasedByOthers,
                                        self(self, {newOpened, meCurrent, destination, meEta - timeDecrement,
                                                    distanceToDestination - timeDecrement, timeLeft - timeDecrement}));
          }
        }
      }

      // Memoize and return
      cache[state] = pressureReleasedFromCurrent + releasedByOthers;
      // fmt::print("{:b} me:{} e:{} meEta:{} elEta:{} time:{}, p:{}\n", opened, pressuredValves[meCurrent],
      //            pressuredValves[elephantCurrent], meEta, elephantEta, timeLeft, cache[state]);
      return pressureReleasedFromCurrent + releasedByOthers;
    };

    return releasedPressure(releasedPressure, {openedValves, start, start, 0, 0, 26});
  }

 private:
  bool isClosed(const uint8_t& valve, const uint16_t& opened) const {
    return !(opened & (1u << valve));
  }

  const std::string VALVE_NAME_PREFIX = "Valve ";
  const std::string FLOW_RATE_PREFIX = "rate=";
  std::tuple<std::unordered_map<std::string, uint8_t>, std::vector<std::string>, std::unordered_map<uint8_t, int>,
             std::unordered_map<uint8_t, std::vector<uint8_t>>>
  parseInput(const std::string& path) const {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    std::unordered_map<std::string, uint8_t> ids;
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
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
