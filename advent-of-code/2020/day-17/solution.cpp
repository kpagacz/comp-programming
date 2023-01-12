#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using Num = int;
struct PointComparator {
  int operator()(const std::tuple<Num, Num, Num>& first, const std::tuple<Num, Num, Num>& second) const {
    auto [x, y, z] = first;
    auto [otherX, otherY, otherZ] = second;
    if (x < otherX) return -1;
    else if (x > otherX) return 1;

    if (y < otherY) return -1;
    else if (y > otherY) return 1;

    if (z < otherZ) return -1;
    else if (z > otherZ) return 1;

    return 0;
  }
};

struct FourDPointComparator {
  int operator()(const std::tuple<Num, Num, Num, Num>& first, const std::tuple<Num, Num, Num, Num>& second) const {
    auto [x, y, z, w] = first;
    auto [otherX, otherY, otherZ, otherW] = second;

    if (x < otherX) return -1;
    else if (x > otherX) return 1;

    auto comp = PointComparator();
    return comp({y, z, w}, {otherY, otherZ, otherW});
  }
};

class Solution {
 public:
  Num part1(const std::string& path, const Num& CYCLES = 6) {
    auto points = parseInput(path);
    Grid grid;
    std::ranges::for_each(points, [&](const auto& point) { addToGrid(point, grid); });

    // main routine
    Num cycles = 0;
    while (cycles++ < CYCLES) {
      std::vector<Point> newPoints;
      Grid newGrid;

      std::ranges::for_each(points, [&](const auto& point) {
        auto neighbourPoints = neighbours(point);
        newPoints.push_back(point);
        newPoints.insert(newPoints.end(), neighbourPoints.begin(), neighbourPoints.end());
      });
      std::ranges::sort(newPoints, [&](const auto& first, const auto& second) {
        auto comp = PointComparator();
        return comp(first, second) == -1;
      });
      auto [pastUnique, last] = std::ranges::unique(newPoints, [&](const auto& first, const auto& second) {
        auto comp = PointComparator();
        return comp(first, second) == 0;
      });
      newPoints.erase(pastUnique, last);

      auto countActiveNeighbours = [&](const auto& point) {
        auto nns = neighbours(point);
        return std::transform_reduce(nns.begin(), nns.end(), (Num)0, std::plus<Num>(),
                                     [&](const auto& neighbour) { return isActive(neighbour, grid) ? 1 : 0; });
      };

      std::ranges::for_each(newPoints, [&](const auto& point) {
        auto activeNeighbours = countActiveNeighbours(point);
        if (isActive(point, grid) && activeNeighbours >= 2 && activeNeighbours <= 3) addToGrid(point, newGrid);
        if (!isActive(point, grid) && activeNeighbours == 3) addToGrid(point, newGrid);
      });

      std::erase_if(newPoints, [&](const auto& point) { return isActive(point, newGrid) == false; });
      points = newPoints;
      grid = newGrid;
    }

    return points.size();
  }

  Num part2(const std::string& path, const Num& CYCLES = 6) {
    auto threeDpoints = parseInput(path);
    std::vector<FourDPoint> points;
    std::ranges::transform(threeDpoints, std::back_inserter(points), [](const auto& point) {
      auto [x, y, z] = point;
      return std::make_tuple(x, y, z, 0);
    });

    FourDGrid grid;
    std::ranges::for_each(points, [&](const auto& point) { addToGrid(point, grid); });

    // main routine
    Num cycles = 0;
    while (cycles++ < CYCLES) {
      std::vector<FourDPoint> newPoints;
      FourDGrid newGrid;

      std::ranges::for_each(points, [&](const auto& point) {
        auto neighbourPoints = neighbours(point);
        newPoints.push_back(point);
        newPoints.insert(newPoints.end(), neighbourPoints.begin(), neighbourPoints.end());
      });
      std::ranges::sort(newPoints, [&](const auto& first, const auto& second) {
        auto comp = FourDPointComparator();
        return comp(first, second) == -1;
      });
      auto [pastUnique, last] = std::ranges::unique(newPoints, [&](const auto& first, const auto& second) {
        auto comp = FourDPointComparator();
        return comp(first, second) == 0;
      });
      newPoints.erase(pastUnique, last);

      auto countActiveNeighbours = [&](const auto& point) {
        auto nns = neighbours(point);
        return std::transform_reduce(nns.begin(), nns.end(), (Num)0, std::plus<Num>(),
                                     [&](const auto& neighbour) { return isActive(neighbour, grid) ? 1 : 0; });
      };

      std::ranges::for_each(newPoints, [&](const auto& point) {
        auto activeNeighbours = countActiveNeighbours(point);
        if (isActive(point, grid) && activeNeighbours >= 2 && activeNeighbours <= 3) addToGrid(point, newGrid);
        if (!isActive(point, grid) && activeNeighbours == 3) addToGrid(point, newGrid);
      });

      std::erase_if(newPoints, [&](const auto& point) { return isActive(point, newGrid) == false; });
      points = newPoints;
      grid = newGrid;
    }

    return points.size();
  }

 private:
  using Grid = std::unordered_map<Num, std::unordered_map<Num, std::unordered_set<Num>>>;
  using Point = std::tuple<Num, Num, Num>;

  using FourDGrid = std::unordered_map<Num, Grid>;
  using FourDPoint = std::tuple<Num, Num, Num, Num>;

  bool isActive(const Point& point, const Grid& grid) {
    const auto& [x, y, z] = point;
    if (!grid.contains(x)) return false;
    if (!grid.at(x).contains(y)) return false;
    if (!grid.at(x).at(y).contains(z)) return false;
    return true;
  }
  bool isActive(const FourDPoint& point, const FourDGrid& grid) {
    const auto& [x, y, z, w] = point;
    if (!grid.contains(x)) return false;
    if (!grid.at(x).contains(y)) return false;
    if (!grid.at(x).at(y).contains(z)) return false;
    if (!grid.at(x).at(y).at(z).contains(w)) return false;
    return true;
  }
  void addToGrid(const Point& point, Grid& grid) {
    const auto& [x, y, z] = point;
    grid[x][y].insert(z);
  }
  void addToGrid(const FourDPoint& point, FourDGrid& grid) {
    const auto& [x, y, z, w] = point;
    grid[x][y][z].insert(w);
  }
  std::vector<Point> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    std::vector<Point> points;
    Num lineIdx = 0;
    while (input >> line) {
      for (auto idx : std::views::iota(0u, line.size()))
        if (line.at(idx) == '#') points.push_back({lineIdx, idx, 0});
      lineIdx++;
    }

    return points;
  }

  std::vector<Point> neighbours(const Point& point) {
    std::vector<Point> neighbours;
    const auto& [x, y, z] = point;

    for (Num deltaX = -1; deltaX <= 1; deltaX++)
      for (Num deltaY = -1; deltaY <= 1; deltaY++)
        for (Num deltaZ = -1; deltaZ <= 1; deltaZ++) neighbours.push_back({x + deltaX, y + deltaY, z + deltaZ});

    std::erase_if(neighbours, [&](const auto& neighbour) {
      const auto& [neighbourX, neighbourY, neighbourZ] = neighbour;
      return x == neighbourX && y == neighbourY && z == neighbourZ;
    });

    return neighbours;
  }
  std::vector<FourDPoint> neighbours(const FourDPoint& point) {
    std::vector<FourDPoint> neighbours;
    const auto& [x, y, z, w] = point;

    for (Num deltaX = -1; deltaX <= 1; deltaX++)
      for (Num deltaY = -1; deltaY <= 1; deltaY++)
        for (Num deltaZ = -1; deltaZ <= 1; deltaZ++)
          for (Num deltaW = -1; deltaW <= 1; deltaW++)
            neighbours.push_back({x + deltaX, y + deltaY, z + deltaZ, w + deltaW});

    std::erase_if(neighbours, [&](const auto& neighbour) {
      const auto& [neighbourX, neighbourY, neighbourZ, neighbourW] = neighbour;
      return x == neighbourX && y == neighbourY && z == neighbourZ && w == neighbourW;
    });

    return neighbours;
  }
};

int main() {
  Solution s;
  std::cout << s.part1("input") << '\n';
  std::cout << s.part2("input") << '\n';
  return 0;
}
