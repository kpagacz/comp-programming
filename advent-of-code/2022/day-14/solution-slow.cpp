#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <set>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
 public:
  uint64_t part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string line;
    std::unordered_map<int, std::set<int>> blockedTiles;
    while (std::getline(input, line)) {
      replaceAll(line, " ->", "");
      replaceAll(line, ",", " ");
      auto points = getPoints(line);
      std::adjacent_find(points.begin(), points.end(), [&](const auto& first, const auto& second) {
        auto [firstX, firstY] = first;
        auto [secondX, secondY] = second;
        for (int x{std::min(firstX, secondX)}; x <= std::max(firstX, secondX); x++)
          for (int y{std::min(firstY, secondY)}; y <= std::max(firstY, secondY); y++) blockedTiles[x].insert(y);
        return false;
      });
    }
    bool hitTheVoid = false;
    int rocksThrown = 0;
    while (!hitTheVoid) {
      bool stuck = false;
      auto position = START;
      while (!stuck) {
        auto [newPosition, didLand] = dropRock(position, blockedTiles);
        if (!didLand) {
          hitTheVoid = true;
          break;
        }
        position = newPosition;
        if (isFree({position.first - 1, position.second + 1}, blockedTiles)) {
          position.first--, position.second++;
          continue;
        }
        if (isFree({position.first + 1, position.second + 1}, blockedTiles)) {
          position.first++, position.second++;
          continue;
        }
        stuck = true;
      }
      if (!hitTheVoid) blockedTiles[position.first].insert(position.second), rocksThrown++;
    }

    return rocksThrown;
  }

  uint64_t part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string line;
    std::unordered_map<int, std::set<int>> blockedTiles;
    int maxY = 0;
    while (std::getline(input, line)) {
      replaceAll(line, " ->", "");
      replaceAll(line, ",", " ");
      auto points = getPoints(line);
      std::adjacent_find(points.begin(), points.end(), [&](const auto& first, const auto& second) {
        auto [firstX, firstY] = first;
        auto [secondX, secondY] = second;
        maxY = std::max(maxY, firstY);
        maxY = std::max(maxY, secondY);
        for (int x{std::min(firstX, secondX)}; x <= std::max(firstX, secondX); x++)
          for (int y{std::min(firstY, secondY)}; y <= std::max(firstY, secondY); y++) blockedTiles[x].insert(y);
        return false;
      });
    }

    int floor = maxY + 2;
    for (int x = 500 - floor - 1; x <= 500 + floor + 1; x++) blockedTiles[x].insert(floor);

    uint64_t totalStones = 0;
    while (!blockedTiles.at(500).contains(0)) {
      bool stuck = false;
      auto position = START;
      while (!stuck) {
        auto [newPosition, didLand] = dropRock(position, blockedTiles);
        assert(didLand);
        position = newPosition;
        if (isFree({position.first - 1, position.second + 1}, blockedTiles)) {
          position.first--, position.second++;
          continue;
        }
        if (isFree({position.first + 1, position.second + 1}, blockedTiles)) {
          position.first++, position.second++;
          continue;
        }
        stuck = true;
      }
      blockedTiles[position.first].insert(position.second), totalStones++;
    }
    return totalStones;
  }

 private:
  const std::pair<int, int> START{500, 0};
  void replaceAll(std::string& text, const std::string& pattern, const std::string& replacement) {
    for (std::size_t it{0}; (it = text.find(pattern, it)) != text.npos; it += replacement.length()) {
      text.replace(it, pattern.length(), replacement.data(), replacement.length());
    }
  }
  std::vector<std::pair<int, int>> getPoints(const std::string& line) {
    std::stringstream in(line);
    std::vector<std::pair<int, int>> points;
    std::string firstPoint, secondPoint;
    while (in >> firstPoint) in >> secondPoint, points.push_back({std::stoi(firstPoint), std::stoi(secondPoint)});
    return points;
  }

  std::pair<std::pair<int, int>, bool> dropRock(const std::pair<int, int>& rockPosition,
                                                const std::unordered_map<int, std::set<int>>& blockedTiles) {
    if (blockedTiles.count(rockPosition.first) == 0) return {{0, 0}, false};
    auto nextStop = blockedTiles.at(rockPosition.first).lower_bound(rockPosition.second);
    if (nextStop == blockedTiles.at(rockPosition.first).end()) return {{0, 0}, false};
    assert(*nextStop != rockPosition.second);
    return {{rockPosition.first, *(nextStop)-1}, true};
  }
  bool isFree(const std::pair<int, int>& position, const std::unordered_map<int, std::set<int>>& blockedTiles) {
    if (blockedTiles.count(position.first) == 0) return true;
    return !blockedTiles.at(position.first).contains(position.second);
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
