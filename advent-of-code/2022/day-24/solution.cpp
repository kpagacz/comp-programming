#include <cassert>
#include <chrono>
#include <fstream>
#include <iostream>
#include <queue>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "utils.cc"

template <class result_t = std::chrono::milliseconds, class clock_t = std::chrono::steady_clock,
          class duration_t = std::chrono::milliseconds>
auto since(std::chrono::time_point<clock_t, duration_t> const& start) {
  return std::chrono::duration_cast<result_t>(clock_t::now() - start);
}

using Blizzard = std::pair<int, int>;                              // offset, step
using Blizzards = std::unordered_map<int, std::vector<Blizzard>>;  // col or row -> Blizzards
class Solution {
 public:
  int part1(const std::string& path) {
    auto [rowBlizzards, colBlizzards, grid] = parseInput(path);
    const std::pair<int, int> START{0, 1};
    const std::pair<int, int> END{grid.size() - 1, grid[0].size() - 2};

    // auto start = std::chrono::steady_clock::now();
    auto answer = findFastest(rowBlizzards, colBlizzards, grid, 0, START, END);
    // std::cout << "Elapsed(ms)=" << since(start).count() << std::endl;
    return answer;
  }

  int part2(const std::string& path) {
    auto [rowBlizzards, colBlizzards, grid] = parseInput(path);
    const std::pair<int, int> START{0, 1};
    const std::pair<int, int> END{grid.size() - 1, grid[0].size() - 2};

    auto toEnd = findFastest(rowBlizzards, colBlizzards, grid, 0, START, END);
    auto backToStart = findFastest(rowBlizzards, colBlizzards, grid, toEnd, END, START);
    auto againToTheEnd = findFastest(rowBlizzards, colBlizzards, grid, backToStart, START, END);
    return againToTheEnd;
  }

 private:
  const std::vector<std::pair<int, int>> moves{{-1, 0}, {0, -1}, {0, 0}, {1, 0}, {0, 1}};
  int findFastest(const Blizzards& rowBlizzards, const Blizzards& colBlizzards,
                  const std::vector<std::vector<char>>& grid, const int& startingTime, const std::pair<int, int>& START,
                  const std::pair<int, int>& END) {
    assert(grid[START.first][START.second] == '.');
    assert(grid[END.first][END.second] == '.');

    using State = std::tuple<int, int, int, int>;  // estimate, actual time passed, current x, current y
    std::priority_queue<State, std::vector<State>, std::greater<State>> astar{std::greater<State>()};
    std::unordered_set<std::tuple<int, int, int>, utils::TupleHash<int, int, int>> cache;

    auto estimateTimeLeft = [&](const auto& currentX, const auto& currentY) {
      return std::abs(END.first - currentX) + std::abs(END.second - currentY);
    };
    auto blizzardPosition = [&, &grid = grid](const auto& blizzard, const auto& time, const auto& isRowBlizzard) {
      const int modulo = isRowBlizzard ? grid[0].size() - 2 : grid.size() - 2;
      int positionAtTime = (blizzard.first - 1) + (blizzard.second * time);
      while (positionAtTime < 0) positionAtTime += modulo;
      positionAtTime = positionAtTime % modulo;
      return positionAtTime + 1;
    };
    auto isInBlizzard = [&, &rowBlizzards = rowBlizzards, &colBlizzards = colBlizzards](const auto& x, const auto& y,
                                                                                        const auto& time) {
      if (rowBlizzards.contains(x))
        for (const auto& blizzard : rowBlizzards.at(x))
          if (y == blizzardPosition(blizzard, time, true)) return true;
      if (colBlizzards.contains(y))
        for (const auto& blizzard : colBlizzards.at(y))
          if (x == blizzardPosition(blizzard, time, false)) return true;
      return false;
    };

    astar.push({estimateTimeLeft(START.first, START.second), startingTime, START.first, START.second});
    while (!astar.empty()) {
      auto [_, actualTime, x, y] = astar.top();
      astar.pop();

      if (cache.contains({actualTime, x, y})) continue;
      else cache.insert({actualTime, x, y});

      if (x == END.first && y == END.second) return actualTime;
      for (const auto& [deltaX, deltaY] : moves) {
        auto newX = x + deltaX, newY = y + deltaY;
        if (!isInGrid(grid, newX, newY)) continue;
        if (isInBlizzard(newX, newY, actualTime + 1)) continue;
        auto estimatedTimeLeftFromNewPosition = estimateTimeLeft(newX, newY);
        astar.push({actualTime + 1 + estimatedTimeLeftFromNewPosition, actualTime + 1, newX, newY});
      }
    }

    return -1;
  }

  bool isInGrid(const std::vector<std::vector<char>>& grid, const int& x, const int& y) {
    return x >= 0 && x < grid.size() && y >= 0 && y < grid[0].size() && grid[x][y] != '#';
  }

  std::tuple<Blizzards, Blizzards, std::vector<std::vector<char>>> parseInput(const std::string& path) {
    Blizzards rowBlizzards, colBlizzards;
    std::vector<std::vector<char>> board;

    std::fstream input(path, std::ios_base::in);
    std::string line;
    while (input >> line) board.push_back(std::vector<char>(line.begin(), line.end()));
    for (int i = 0; i < board.size(); i++)
      for (int j = 0; j < board[0].size(); j++) {
        switch (board[i][j]) {
          case '>':
            rowBlizzards[i].push_back({j, 1});
            break;
          case '<':
            rowBlizzards[i].push_back({j, -1});
            break;
          case 'v':
            colBlizzards[j].push_back({i, 1});
            break;
          case '^':
            colBlizzards[j].push_back({i, -1});
            break;
        }
      }
    return {rowBlizzards, colBlizzards, board};
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
