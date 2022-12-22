#include <cassert>
#include <fstream>
#include <iostream>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>

#include "utils.cc"

enum Direction { RIGHT, DOWN, LEFT, UP };
constexpr char WRAP_CHAR = 'W';

void printGrid(const std::vector<std::vector<char>>& grid, const int& x, const int& y, const Direction& dir) {
  for (int i = 0; i < grid.size(); i++) {
    for (int j = 0; j < grid[0].size(); j++)
      if (x == i && y == j) {
        switch (dir) {
          case UP:
            std::cout << "U";
            break;
          case DOWN:
            std::cout << "D";
            break;
          case RIGHT:
            std::cout << "R";
            break;
          case LEFT:
            std::cout << "L";
            break;
        }
      } else std::cout << grid[i][j];
    std::cout << '\n';
  }
  std::cout << "\n\n";
}

class Solution {
 public:
  int part1(const std::string& path) {
    auto [grid, instructions] = parseInput(path);

    std::tuple<int, int, Direction> position;
    auto startingY = std::find(grid[0].begin(), grid[0].end(), '.') - grid[0].begin();
    position = {0, startingY, RIGHT};

    const int colSize = grid[0].size(), rowSize = grid.size();

    auto get = [&, &grid = grid](const auto& x, const auto& y) { return grid[x % rowSize][y % colSize]; };
    auto move = [&, &grid = grid](const Instruction& instruction, std::tuple<int, int, Direction>& position) {
      std::get<0>(position) += rowSize;
      std::get<1>(position) += colSize;
      std::pair<int, int> step;
      switch (std::get<2>(position)) {
        case UP:
          step = {-1, 0};
          break;
        case DOWN:
          step = {1, 0};
          break;
        case RIGHT:
          step = {0, 1};
          break;
        case LEFT:
          step = {0, -1};
          break;
        default:
          throw std::invalid_argument("Unrecognized direction");
      }

      for (int i = 0; i < std::get<0>(instruction); i++) {
        auto& [x, y, _] = position;
        if (get(x + step.first, y + step.second) == '.') x += step.first, y += step.second;
        else if (get(x + step.first, y + step.second) == '#') break;
        else {
          auto wrappedX = x, wrappedY = y;
          while (get(wrappedX + step.first, wrappedY + step.second) == 'W')
            wrappedX += step.first, wrappedY += step.second;
          if (get(wrappedX + step.first, wrappedY + step.second) == '.')
            x = wrappedX + step.first, y = wrappedY + step.second;
          else break;
        }
      }

      if (std::get<1>(instruction).has_value()) {
        switch (std::get<1>(instruction).value()) {
          case 'R':
            std::get<2>(position) = Direction((std::get<2>(position) + 1) % 4);
            break;
          case 'L':
            std::get<2>(position) = Direction((std::get<2>(position) + 4 - 1) % 4);
            break;
        }
      }
    };

    for (const auto& instruction : instructions) move(instruction, position);
    auto [x, y, d] = position;

    auto row = std::get<0>(position) % rowSize + 1;
    auto col = std::get<1>(position) % colSize + 1;
    return 1000 * row + 4 * col + std::get<2>(position);
  }

  int part2(const std::string& path) {
    auto [grid, instructions] = parseInput(path);

    std::tuple<int, int, Direction> position;
    auto startingY = std::find(grid[0].begin(), grid[0].end(), '.') - grid[0].begin();
    position = {0, startingY, RIGHT};

    auto rowSize = grid.size(), colSize = grid[0].size();

    auto stepForDirection = [&](const Direction& direction) {
      switch (direction) {
        case UP:
          return std::pair<int, int>{-1, 0};
        case DOWN:
          return std::pair<int, int>{1, 0};
        case RIGHT:
          return std::pair<int, int>{0, 1};
        case LEFT:
          return std::pair<int, int>{0, -1};
        default:
          throw std::invalid_argument("Unrecognized direction");
      }
    };
    auto isWrapping = [&, &grid = grid](const auto& x, const auto& y) {
      return x < 0 || x >= grid.size() || y < 0 || y >= grid[0].size() || grid[x][y] == 'W';
    };
    const std::unordered_map<std::pair<int, int>, int, utils::TupleHash<int, int>> sectorMapping{
        {{0, 0}, 0}, {{0, 1}, 1}, {{0, 2}, 2}, {{1, 0}, 3}, {{1, 1}, 4},  {{1, 2}, 5},
        {{2, 0}, 6}, {{2, 1}, 7}, {{2, 2}, 8}, {{3, 0}, 9}, {{3, 1}, 10}, {{3, 2}, 11}};
    auto sectorRowSize = rowSize / 4, sectorColSize = colSize / 3;
    assert(sectorRowSize == sectorColSize);
    auto getSector = [&](const auto& x, const auto& y) {
      return sectorMapping.at({x / sectorRowSize, y / sectorColSize});
    };
    auto getNextWrappedPosition = [&, &grid = grid](const auto& position) {
      const auto& [x, y, direction] = position;
      auto sector = getSector(x, y);

      if (sector == 1 && direction == LEFT) {
        return std::tuple<int, int, Direction>{2 * sectorRowSize + (sectorRowSize - 1 - x), 0, RIGHT};
      } else if (sector == 1 && direction == UP) {
        return std::tuple<int, int, Direction>{3 * sectorRowSize + y - sectorColSize, 0, RIGHT};
      } else if (sector == 2 && direction == UP) {
        return std::tuple<int, int, Direction>{4 * sectorRowSize - 1, y - 2 * sectorColSize, UP};
      } else if (sector == 2 && direction == RIGHT) {
        return std::tuple<int, int, Direction>{2 * sectorRowSize + (sectorColSize - 1 - x), 2 * sectorColSize - 1,
                                               LEFT};
      } else if (sector == 2 && direction == DOWN) {
        return std::tuple<int, int, Direction>{sectorRowSize + y - 2 * sectorColSize, 2 * sectorColSize - 1, LEFT};
      } else if (sector == 4 && direction == LEFT) {
        return std::tuple<int, int, Direction>{2 * sectorRowSize, x - sectorRowSize, DOWN};
      } else if (sector == 4 && direction == RIGHT) {
        return std::tuple<int, int, Direction>{sectorRowSize - 1, 2 * sectorColSize + x - sectorRowSize, UP};
      } else if (sector == 6 && direction == LEFT) {
        return std::tuple<int, int, Direction>{sectorRowSize - 1 - (x - 2 * sectorRowSize), sectorColSize, RIGHT};
      } else if (sector == 6 && direction == UP) {
        return std::tuple<int, int, Direction>{sectorRowSize + y, sectorColSize, RIGHT};
      } else if (sector == 7 && direction == RIGHT) {
        return std::tuple<int, int, Direction>{sectorRowSize - 1 - (x - 2 * sectorRowSize), 3 * sectorColSize - 1,
                                               LEFT};
      } else if (sector == 7 && direction == DOWN) {
        return std::tuple<int, int, Direction>{3 * sectorRowSize + y - sectorColSize, sectorColSize - 1, LEFT};
      } else if (sector == 9 && direction == LEFT) {
        return std::tuple<int, int, Direction>{0, sectorColSize + (x - 3 * sectorRowSize), DOWN};
      } else if (sector == 9 && direction == RIGHT) {
        return std::tuple<int, int, Direction>{3 * sectorRowSize - 1, sectorColSize + x - 3 * sectorRowSize, UP};
      } else if (sector == 9 && direction == DOWN) {
        return std::tuple<int, int, Direction>{0, 2 * sectorColSize + y, DOWN};
      } else {
        throw std::invalid_argument("Cannot wrap this shit");
      }
    };

    auto move = [&, &grid = grid](const Instruction& instruction, std::tuple<int, int, Direction>& position) {
      for (int i = 0; i < std::get<0>(instruction); i++) {
        auto step = stepForDirection(std::get<2>(position));
        auto& [x, y, direction] = position;
        auto nextX = x + step.first, nextY = y + step.second;
        Direction nextDirection = direction;
        if (isWrapping(nextX, nextY)) std::tie(nextX, nextY, nextDirection) = getNextWrappedPosition(position);
        if (grid[nextX][nextY] != '#') x = nextX, y = nextY, direction = nextDirection;
        else break;
      }

      if (std::get<1>(instruction).has_value()) {
        switch (std::get<1>(instruction).value()) {
          case 'R':
            std::get<2>(position) = Direction((std::get<2>(position) + 1) % 4);
            break;
          case 'L':
            std::get<2>(position) = Direction((std::get<2>(position) + 4 - 1) % 4);
            break;
        }
      }
    };

    for (const auto& instruction : instructions) move(instruction, position);
    auto [x, y, d] = position;

    auto row = std::get<0>(position) % rowSize + 1;
    auto col = std::get<1>(position) % colSize + 1;
    return 1000 * row + 4 * col + std::get<2>(position);
  }

 private:
  using Grid = std::vector<std::vector<char>>;
  using Instruction = std::pair<int, std::optional<char>>;
  using Instructions = std::vector<Instruction>;

  std::pair<Grid, Instructions> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    Grid grid;
    Instructions instructions;
    std::size_t maxSize = 0;
    while (std::getline(input, line)) {
      if (line == "") {
        std::getline(input, line);
        int move;
        char turn;
        std::size_t it = 0, nextTurn;
        while ((nextTurn = line.find_first_not_of("0123456789", it)) != line.npos) {
          move = std::stoi(line.substr(it, nextTurn));
          turn = line[nextTurn];
          instructions.push_back({move, std::optional<char>{turn}});
          it = nextTurn + 1;
        }
        instructions.push_back({std::stoi(line.substr(it)), std::nullopt});
        continue;
      }
      maxSize = std::max(maxSize, line.size());
      grid.push_back(std::vector<char>(line.begin(), line.end()));
    }
    for (auto& row : grid) row.resize(maxSize, WRAP_CHAR);
    for (auto& row : grid) std::replace(row.begin(), row.end(), ' ', WRAP_CHAR);

    return {grid, instructions};
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
