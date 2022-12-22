#include <cassert>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

#include "utils.cc"

enum Direction { RIGHT, DOWN, LEFT, UP };

int main() {
  // W..
  // W.W
  // ..W
  // .WW
  const std::string input1 =
      "W..\n"
      "W.W\n"
      "..W\n"
      ".WW\n";
  const std::string input2 =
      "WW....\n"
      "WW....\n"
      "WW..WW\n"
      "WW..WW\n"
      "....WW\n"
      "....WW\n"
      "..WWWW\n"
      "..WWWW\n";
  std::stringstream ss(input1);
  std::stringstream ss2(input2);
  std::vector<std::vector<char>> grid;
  std::vector<std::vector<char>> grid2;
  std::string line;
  while (ss >> line) grid.push_back(std::vector<char>(line.begin(), line.end()));
  while (ss2 >> line) grid2.push_back(std::vector<char>(line.begin(), line.end()));
  grid = grid2;

  // logic
  auto rowSize = grid.size(), colSize = grid[0].size();
  std::cout << "Row size:" << rowSize << " colSize:" << colSize << '\n';

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
  std::cout << "Sector row size:" << sectorRowSize << " sector col size:" << sectorColSize << '\n';
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
      return std::tuple<int, int, Direction>{2 * sectorRowSize + (sectorColSize - 1 - x), 2 * sectorColSize - 1, LEFT};
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
      return std::tuple<int, int, Direction>{sectorRowSize - 1 - (x - 2 * sectorRowSize), 3 * sectorColSize - 1, LEFT};
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

  std::vector<std::pair<std::tuple<int, int, Direction>, std::tuple<int, int, Direction>>> cases;
  // Input1 cases
  // cases.push_back({{0, 1, LEFT}, {2, 0, RIGHT}});
  // cases.push_back({{0, 1, UP}, {3, 0, RIGHT}});
  // cases.push_back({{0, 2, UP}, {3, 0, UP}});
  // cases.push_back({{1, 1, LEFT}, {2, 0, DOWN}});
  // cases.push_back({{1, 1, RIGHT}, {0, 2, UP}});
  // cases.push_back({{2, 0, UP}, {1, 1, RIGHT}});
  // cases.push_back({{2, 0, LEFT}, {0, 1, RIGHT}});
  // cases.push_back({{2, 1, RIGHT}, {0, 2, LEFT}});
  // cases.push_back({{2, 1, DOWN}, {3, 0, LEFT}});
  // cases.push_back({{3, 0, LEFT}, {0, 1, DOWN}});
  // cases.push_back({{3, 0, DOWN}, {0, 2, DOWN}});
  // cases.push_back({{3, 0, RIGHT}, {2, 1, UP}});

  // Input2 cases
  cases.push_back({{0, 2, LEFT}, {5, 0, RIGHT}});
  cases.push_back({{0, 2, UP}, {6, 0, RIGHT}});
  cases.push_back({{0, 4, UP}, {7, 0, UP}});
  cases.push_back({{2, 2, LEFT}, {4, 0, DOWN}});
  cases.push_back({{2, 3, RIGHT}, {1, 4, UP}});
  cases.push_back({{4, 0, UP}, {2, 2, RIGHT}});
  cases.push_back({{4, 0, LEFT}, {1, 2, RIGHT}});
  cases.push_back({{4, 3, RIGHT}, {1, 5, LEFT}});
  cases.push_back({{5, 3, DOWN}, {7, 1, LEFT}});
  cases.push_back({{6, 0, LEFT}, {0, 2, DOWN}});
  cases.push_back({{7, 0, DOWN}, {0, 4, DOWN}});
  cases.push_back({{6, 1, RIGHT}, {5, 2, UP}});
  for (const auto& [source, expectedDestination] : cases) {
    auto step = stepForDirection(std::get<2>(source));
    const auto& [x, y, d] = source;
    std::cout << "Source is:" << x << " " << y << " ";
    switch (d) {
      case RIGHT:
        std::cout << "R";
        break;
      case LEFT:
        std::cout << "L";
        break;
      case UP:
        std::cout << "U";
        break;
      case DOWN:
        std::cout << "D";
        break;
    }
    std::cout << '\n';
    std::cout << "Step:" << step.first << " " << step.second << '\n';
    std::cout << "Sector for source is:" << getSector(x, y) << '\n';
    assert(isWrapping(x + step.first, y + step.second));
    const auto& [expectedX, expectedY, expectedD] = expectedDestination;
    auto [resultX, resultY, resultD] = getNextWrappedPosition(source);
    std::cout << "result pos:" << resultX << " " << resultY << " " << resultD << '\n';
    assert(resultX == expectedX);
    assert(resultY == expectedY);
    assert(resultD == expectedD);
    std::cout << '\n';
  }

  return 0;
}
