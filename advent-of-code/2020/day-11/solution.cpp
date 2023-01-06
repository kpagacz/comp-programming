#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <string>
#include <vector>

using NumType = int;
class Solution {
 public:
  NumType part1(const std::string& path) {
    auto layout = parseLayout(path);
    auto isValidCoord = [&](const std::pair<NumType, NumType>& coord) {
      return coord.first >= 0 && coord.first < layout.size() && coord.second >= 0 && coord.second < layout[0].size();
    };
    auto countOccupiedNeighbours = [&](const std::pair<NumType, NumType>& seat) {
      return std::transform_reduce(cardinalDirections.begin(), cardinalDirections.end(), 0, std::plus<int>(),
                                   [&](const auto& direction) {
                                     auto x = seat.first + direction.first, y = seat.second + direction.second;
                                     if (!isValidCoord({x, y})) return 0;
                                     return layout[x][y] == '#' ? 1 : 0;
                                   });
    };
    NumType turns = 0;
    bool changed = true;
    while (changed) {
      turns++;
      auto newLayout = layout;
      changed = false;
      for (const auto& i : std::views::iota(0u, layout.size()))
        for (const auto& j : std::views::iota(0u, layout[0].size())) {
          auto occupiedNeighbours = countOccupiedNeighbours({i, j});
          if (layout[i][j] == 'L' && occupiedNeighbours == 0) newLayout[i][j] = '#', changed = true;
          else if (layout[i][j] == '#' && occupiedNeighbours >= 4) newLayout[i][j] = 'L', changed = true;
        }
      layout = newLayout;
    }

    return std::transform_reduce(layout.begin(), layout.end(), 0, std::plus<int>(), [](const auto& row) {
      return std::transform_reduce(row.begin(), row.end(), 0, std::plus<int>(),
                                   [](const auto& place) { return place == '#' ? 1 : 0; });
    });
  }

  NumType part2(const std::string& path) {
    auto layout = parseLayout(path);
    auto isValidCoord = [&](const std::pair<NumType, NumType>& coord) {
      return coord.first >= 0 && coord.first < layout.size() && coord.second >= 0 && coord.second < layout[0].size();
    };
    auto countOccupiedNeighbours = [&](const std::pair<NumType, NumType>& seat) {
      return std::transform_reduce(cardinalDirections.begin(), cardinalDirections.end(), 0, std::plus<int>(),
                                   [&](const auto& direction) {
                                     auto x = seat.first + direction.first, y = seat.second + direction.second;
                                     while (isValidCoord({x, y})) {
                                       if (layout[x][y] == 'L') return 0;
                                       else if (layout[x][y] == '#') return 1;
                                       x += direction.first, y += direction.second;
                                     }
                                     return 0;
                                   });
    };
    NumType turns = 0;
    bool changed = true;
    while (changed) {
      turns++;
      auto newLayout = layout;
      changed = false;
      for (const auto& i : std::views::iota(0u, layout.size()))
        for (const auto& j : std::views::iota(0u, layout[0].size())) {
          auto occupiedNeighbours = countOccupiedNeighbours({i, j});
          if (layout[i][j] == 'L' && occupiedNeighbours == 0) newLayout[i][j] = '#', changed = true;
          else if (layout[i][j] == '#' && occupiedNeighbours >= 5) newLayout[i][j] = 'L', changed = true;
        }
      layout = newLayout;
    }

    return std::transform_reduce(layout.begin(), layout.end(), 0, std::plus<int>(), [](const auto& row) {
      return std::transform_reduce(row.begin(), row.end(), 0, std::plus<int>(),
                                   [](const auto& place) { return place == '#' ? 1 : 0; });
    });
  }

 private:
  const std::vector<std::pair<NumType, NumType>> cardinalDirections{{-1, -1}, {-1, 0}, {-1, 1}, {0, -1},
                                                                    {0, 1},   {1, -1}, {1, 0},  {1, 1}};
  using Grid = std::vector<std::vector<char>>;
  Grid parseLayout(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    Grid layout;
    while (input >> line) layout.push_back(std::vector<char>(line.begin(), line.end()));
    return layout;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
