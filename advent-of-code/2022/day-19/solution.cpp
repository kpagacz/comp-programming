#include <algorithm>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <numeric>
#include <stack>
#include <string>
#include <vector>

enum Material { Ore, Clay, Obsidian, Geode };

struct Blueprint {
  int oreRobot, clayRobot, index;
  std::pair<int, int> obsidianRobot, geodeRobot;

  Blueprint(int _index, int _oreRobot, int _clayRobot, std::pair<int, int> _obsidianRobot,
            std::pair<int, int> _geodeRobot)
      : index(_index),
        oreRobot(_oreRobot),
        clayRobot(_clayRobot),
        obsidianRobot(_obsidianRobot),
        geodeRobot(_geodeRobot) {}

  int maxCost(Material material) const {
    int max = 0;
    switch (material) {
      case Ore:
        max = std::max(clayRobot, oreRobot);
        max = std::max(obsidianRobot.first, max);
        max = std::max(geodeRobot.first, max);
        return max;
      case Clay:
        return obsidianRobot.second;
      case Obsidian:
        return geodeRobot.second;
      case Geode:
        return 0;
    }
  }

  void print(std::ostream& out) const {
    out << "Blueprint. Ore robot cost: " << oreRobot << " ore. Clay robot: " << clayRobot
        << " ore. Obsidian robot: " << obsidianRobot.first << " ore and " << obsidianRobot.second
        << " clay. Geode robot: " << geodeRobot.first << " ore and " << geodeRobot.second << " obsidian.\n";
  }
};

constexpr int TIME_LIMIT = 24;
class BlueprintQuality {
 public:
  static int get(const Blueprint& blueprint) {
    std::stack<State> dfs;
    dfs.push({1, 0, 0, 0, 0, 0, 0, TIME_LIMIT});
    int maxGeodes = 0;

    while (!dfs.empty()) {
      auto state = dfs.top();
      auto [oreR, clayR, obsidianR, ore, clay, obsidian, geode, timeLeft] = dfs.top();
      dfs.pop();
      if (timeLeft <= 1) {
        maxGeodes = std::max(maxGeodes, stored(Geode, state));
        continue;
      }

      print(state);
      for (const auto& material : {Geode, Obsidian, Clay, Ore}) {
        if (productionEqualsMaxNeed(material, state, blueprint)) continue;
        if (!canBuildRobot(material, state)) continue;
        auto daysToSkip = daysToBuildRobot(material, state, blueprint);
        auto newState = advanceDays(daysToSkip, state);
        // print(newState);
        newState = buyRobot(material, newState, blueprint);
        dfs.push(newState);
      }
    }

    return maxGeodes * blueprint.index;
  }

 private:
  using State = std::tuple<int, int, int, int, int, int, int,
                           int>;  // oreR, clayR, obsidianR, ore, clay, obsidian, geode, timeLeft
  static void print(const State& state) {
    std::cout << "State. Ore Rs: " << std::get<0>(state) << " Clay Rs:" << std::get<1>(state)
              << " Obsidian Rs:" << std::get<2>(state) << " stored: " << stored(Ore, state) << " "
              << stored(Clay, state) << " " << stored(Obsidian, state) << " " << stored(Geode, state)
              << " time: " << std::get<7>(state) << '\n';
  }
  static bool canBuildRobot(const Material& material, const State& state) {
    switch (material) {
      case Ore:
        return true;
      case Clay:
        return true;
      case Obsidian:
        return std::get<1>(state) > 0;
      case Geode:
        return std::get<2>(state) > 0;
    }
  }
  static State buyRobot(const Material& material, const State& state, const Blueprint& b) {
    switch (material) {
      case Ore:
        return {std::get<0>(state) + 1, std::get<1>(state), std::get<2>(state), std::get<3>(state) - b.oreRobot,
                std::get<4>(state),     std::get<5>(state), std::get<6>(state), std::get<7>(state)};
      case Clay:
        return {std::get<0>(state), std::get<1>(state) + 1, std::get<2>(state), std::get<3>(state) - b.clayRobot,
                std::get<4>(state), std::get<5>(state),     std::get<6>(state), std::get<7>(state)};
      case Obsidian:
        return {std::get<0>(state),
                std::get<1>(state),
                std::get<2>(state) + 1,
                std::get<3>(state) - b.obsidianRobot.first,
                std::get<4>(state) - b.obsidianRobot.second,
                std::get<5>(state),
                std::get<6>(state),
                std::get<7>(state)};
      case Geode:
        return {std::get<0>(state),
                std::get<1>(state),
                std::get<2>(state),
                std::get<3>(state) - b.geodeRobot.first,
                std::get<4>(state),
                std::get<5>(state) - b.geodeRobot.second,
                std::get<6>(state) + std::get<7>(state) - 1,
                std::get<7>(state)};
    }
  }

  static int daysToBuildRobot(const Material& material, const State& state, const Blueprint& blueprint) {
    int oreWait, clayWait, obsidianWait;
    auto roundUp = [&](const auto& divident, const auto& divisor) {
      return divident % divisor == 0 ? divident / divisor : divident / divisor + 1;
    };
    switch (material) {
      case Ore:
        return roundUp(std::max(blueprint.oreRobot - stored(Ore, state), 0), std::get<0>(state)) + 1;
      case Clay:
        return roundUp(std::max(blueprint.clayRobot - stored(Clay, state), 0), std::get<0>(state)) + 1;
      case Obsidian:
        oreWait = roundUp(std::max(blueprint.obsidianRobot.first - stored(Ore, state), 0), std::get<0>(state));
        clayWait = roundUp(std::max(blueprint.obsidianRobot.second - stored(Clay, state), 0), std::get<1>(state));
        return std::max(oreWait, clayWait) + 1;
      case Geode:
        oreWait = roundUp(std::max(blueprint.geodeRobot.first - stored(Ore, state), 0), std::get<0>(state));
        obsidianWait = roundUp(std::max(0, blueprint.geodeRobot.second - stored(Obsidian, state)), std::get<2>(state));
        return std::max(oreWait, obsidianWait) + 1;
    }
  }

  static State advanceDays(const int& days, const State& state) {
    return {std::get<0>(state),
            std::get<1>(state),
            std::get<2>(state),
            stored(Ore, state) + days * std::get<0>(state),
            stored(Clay, state) + days * std::get<1>(state),
            stored(Obsidian, state) + days * std::get<2>(state),
            std::get<6>(state),
            std::get<7>(state) - days};
  }

  static int stored(const Material& material, const State& state) {
    switch (material) {
      case Ore:
        return std::get<3>(state);
      case Clay:
        return std::get<4>(state);
      case Obsidian:
        return std::get<5>(state);
      case Geode:
        return std::get<6>(state);
    }
  }

  static bool productionEqualsMaxNeed(const Material& material, const State& state, const Blueprint& blueprint) {
    switch (material) {
      case Ore:
        return std::get<0>(state) == blueprint.maxCost(material);
      case Clay:
        return std::get<1>(state) == blueprint.maxCost(material);
      case Obsidian:
        return std::get<2>(state) == blueprint.maxCost(material);
      default:
        return false;
    }
  }
};

class Solution {
 public:
  int part1(const std::string& path) {
    auto blueprints = parseInput(path);
    for (const auto& b : blueprints) b.print(std::cout);
    int sum = 0;

    for (const auto& blueprint : blueprints) {
      auto quality = BlueprintQuality::get(blueprint);
      std::cout << "QUALITY: " << quality << '\n';
      sum += quality;
    }
    return sum;
  }

 private:
  std::vector<Blueprint> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::vector<Blueprint> blueprints;
    std::string line;
    while (std::getline(input, line)) {
      int oreRobot, clayRobot, obsidianOreRobot, obsidianClayRobot, geodeOreRobot, geodeObsidianRobot, index;
      const auto assigned = std::sscanf(
          line.c_str(),
          "Blueprint %i: Each ore robot costs %i ore. Each clay robot costs %i ore. Each obsidian robot costs %i ore "
          "and %i clay. Each geode robot costs %i ore and %i obsidian.",
          &index, &oreRobot, &clayRobot, &obsidianOreRobot, &obsidianClayRobot, &geodeOreRobot, &geodeObsidianRobot);
      assert(assigned != EOF && assigned == 7);
      Blueprint b(index, oreRobot, clayRobot, {obsidianOreRobot, obsidianClayRobot},
                  {geodeOreRobot, geodeObsidianRobot});
      blueprints.push_back(b);
    }
    return blueprints;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("test") << '\n';
  return 0;
}
