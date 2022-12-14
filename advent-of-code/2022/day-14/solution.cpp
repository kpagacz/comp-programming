#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stack>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

class Solution {
 public:
  uint64_t part1(const std::string& pathToInput) {
    auto [blockedTiles, bottom] = getStartingTunnels(pathToInput);
    std::stack<std::pair<int, int>> passPoints;
    passPoints.push(START);
    uint64_t rocksThrown = 0;
    while (!passPoints.empty()) {
      auto top = passPoints.top();
      if (top.second == bottom) break;
      bool passed = false;
      for (auto newPosition : std::vector<std::pair<int, int>>{
               {top.first + 1, top.second + 1}, {top.first - 1, top.second + 1}, {top.first, top.second + 1}}) {
        if (isFree(newPosition, blockedTiles)) passPoints.push(newPosition), passed = true;
      }
      if (!passed) blockedTiles[top.first].insert(top.second), rocksThrown++, passPoints.pop();
    }
    return rocksThrown;
  }

  uint64_t part2(const std::string& pathToInput) {
    auto [blockedTiles, bottom] = getStartingTunnels(pathToInput);
    bottom += 2;
    std::stack<std::pair<int, int>> passPoints;
    passPoints.push(START);
    uint64_t rocksThrown = 0;
    while (!passPoints.empty()) {
      auto top = passPoints.top();
      if (top.second == bottom) {
        blockedTiles[top.first].insert(top.second);
        passPoints.pop();
        continue;
      }
      bool passed = false;
      for (auto newPosition : std::vector<std::pair<int, int>>{
               {top.first + 1, top.second + 1}, {top.first - 1, top.second + 1}, {top.first, top.second + 1}}) {
        if (isFree(newPosition, blockedTiles)) passPoints.push(newPosition), passed = true;
      }
      if (!passed) blockedTiles[top.first].insert(top.second), rocksThrown++, passPoints.pop();
    }
    return rocksThrown;
  }

 private:
  const std::pair<int, int> START{500, 0};
  std::pair<std::unordered_map<int, std::unordered_set<int>>, int> getStartingTunnels(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string line;
    std::unordered_map<int, std::unordered_set<int>> blockedTiles;
    int bottom = 0;
    while (std::getline(input, line)) {
      replaceAll(line, " ->", "");
      replaceAll(line, ",", " ");
      auto points = getPoints(line);
      std::adjacent_find(points.begin(), points.end(), [&](const auto& first, const auto& second) {
        auto [firstX, firstY] = first;
        auto [secondX, secondY] = second;
        bottom = std::max(bottom, firstY);
        bottom = std::max(bottom, secondY);
        for (int x{std::min(firstX, secondX)}; x <= std::max(firstX, secondX); x++)
          for (int y{std::min(firstY, secondY)}; y <= std::max(firstY, secondY); y++) blockedTiles[x].insert(y);
        return false;
      });
    }
    return {blockedTiles, bottom};
  }

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

  bool isFree(const std::pair<int, int>& position,
              const std::unordered_map<int, std::unordered_set<int>>& blockedTiles) {
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
