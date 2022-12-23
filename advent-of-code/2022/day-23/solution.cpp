#include <algorithm>
#include <cassert>
#include <deque>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "utils.cc"

using Positions = std::unordered_set<std::pair<int, int>, utils::TupleHash<int, int>>;
class Solution {
 public:
  int part1(const std::string& path) {
    auto elves = parseInput(path);
    Positions proposedMoves;
    auto initialSize = elves.size();

    int ROUNDS = 10;
    while (ROUNDS--) {
      proposedMoves.clear();

      for (const auto& elf : elves) {
        auto proposedMove = proposeMove(elf, elves);
        if (!proposedMoves.contains(proposedMove)) {
          proposedMoves.insert(proposedMove);
          continue;
        }
        proposedMoves.insert(elf);
        proposedMoves.erase(proposedMove);
        auto delta = subtract(proposedMove, elf);
        proposedMoves.insert(add(proposedMove, delta));
      }

      rotateConsideredMoves();
      elves = proposedMoves;
    }
    auto [left, right, top, bottom] = findBorders(elves);

    assert(initialSize == elves.size());
    return (right - left + 1) * (bottom - top + 1) - elves.size();
  }

  int part2(const std::string& path) {
    auto elves = parseInput(path);
    Positions proposedMoves;
    consideredMoves = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

    int rounds = 0;
    while (true) {
      rounds++;
      proposedMoves.clear();

      for (const auto& elf : elves) {
        auto proposedMove = proposeMove(elf, elves);
        if (!proposedMoves.contains(proposedMove)) {
          proposedMoves.insert(proposedMove);
          continue;
        }
        proposedMoves.insert(elf);
        proposedMoves.erase(proposedMove);
        auto delta = subtract(proposedMove, elf);
        proposedMoves.insert(add(proposedMove, delta));
      }

      if (arePositionsEqual(elves, proposedMoves)) return rounds;
      rotateConsideredMoves();
      elves = proposedMoves;
    }

    return -1;
  }

 private:
  const std::vector<std::pair<int, int>> surroundings{{-1, -1}, {-1, 0}, {-1, 1}, {0, -1},
                                                      {0, 1},   {1, -1}, {1, 0},  {1, 1}};
  std::deque<std::pair<int, int>> consideredMoves{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
  const std::vector<std::pair<int, int>> rowScan{{0, -1}, {0, 0}, {0, 1}}, colScan{{-1, 0}, {0, 0}, {1, 0}};

  void printGrid(const Positions& positions) {
    auto [left, right, top, bottom] = findBorders(positions);
    for (int x = top; x <= bottom; x++) {
      for (int y = left; y <= right; y++)
        if (positions.contains({x, y})) std::cout << '#';
        else std::cout << '.';
      std::cout << "\n";
    }
    std::cout << "\n";
  }

  bool arePositionsEqual(const Positions& first, const Positions& second) {
    return first == second;
  }

  std::tuple<int, int, int, int> findBorders(const Positions& elves) {
    int leftBorder = INT32_MAX, rightBorder = INT32_MIN, topBorder = INT32_MAX, bottomBorder = INT32_MIN;
    for (const auto& [x, y] : elves) {
      leftBorder = std::min(leftBorder, y);
      rightBorder = std::max(rightBorder, y);
      topBorder = std::min(topBorder, x);
      bottomBorder = std::max(bottomBorder, x);
    }
    return {leftBorder, rightBorder, topBorder, bottomBorder};
  }

  void rotateConsideredMoves() {
    std::rotate(consideredMoves.begin(), consideredMoves.begin() + 1, consideredMoves.end());
  }

  std::pair<int, int> proposeMove(const std::pair<int, int>& elf, const Positions& elves) {
    bool hasNeighbours = false;
    for (const auto& dir : surroundings) hasNeighbours = hasNeighbours || elves.contains(add(dir, elf));
    if (!hasNeighbours) return elf;

    for (const auto& side : consideredMoves) {
      auto isElf = false;
      auto& scanningArray = side.first == 0 ? colScan : rowScan;
      for (const auto& scan : scanningArray) {
        auto scannedPos = add(scan, side);
        scannedPos = add(scannedPos, elf);
        isElf = isElf || elves.contains(scannedPos);
      }
      if (isElf) continue;
      return add(elf, side);
    }

    return elf;
  }

  std::pair<int, int> add(const std::pair<int, int>& first, const std::pair<int, int>& second) {
    return {first.first + second.first, first.second + second.second};
  }

  std::pair<int, int> subtract(const std::pair<int, int>& first, const std::pair<int, int>& second) {
    return {first.first - second.first, first.second - second.second};
  }

  Positions parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    Positions elves;
    int row = 0;
    while (input >> line) {
      int elf = -1;
      while ((elf = line.find('#', elf + 1)) != line.npos) elves.insert({row, elf});
      row++;
    }

    return elves;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
