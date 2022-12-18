#include <algorithm>
#include <cassert>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <numeric>
#include <queue>
#include <string>
#include <unordered_set>
#include <vector>

struct PointHash {
  std::size_t operator()(const std::tuple<int, int, int>& points) const {
    std::size_t hash{0};
    hash = std::hash<int>()(std::get<0>(points));
    hash ^= std::hash<int>()(std::get<1>(points)) << 10;
    hash ^= std::hash<int>()(std::get<2>(points)) << 20;
    return hash;
  }
};

class Solution {
 public:
  uint64_t solve(const std::string& path) {
    auto points = read(path);
    // for (const auto& [x, y, z] : points) std::cout << x << " " << y << " " << z << '\n';

    uint64_t exposedSides = 6 * points.size();
    uint64_t connections = 0;
    for (const auto& point : points)
      for (const auto& direction : directions)
        if (points.contains(add(point, direction))) connections++;

    exposedSides -= connections;
    std::cout << "Part 1: " << exposedSides << '\n';

    exposedSides = 0;
    const int LOWER_LIMIT = 0, UPPER_LIMIT = 20;
    auto isValidDestination = [&](const Point& destination) {
      const auto& [x, y, z] = destination;
      return x >= LOWER_LIMIT && x <= UPPER_LIMIT && y >= LOWER_LIMIT && y <= UPPER_LIMIT && z >= LOWER_LIMIT &&
             z <= UPPER_LIMIT && !points.contains(destination);
    };
    auto countTouchingSides = [&](const Point& cube) {
      int touchingSides = 0;
      for (const auto& direction : directions)
        if (points.contains(add(cube, direction))) touchingSides++;
      return touchingSides;
    };
    std::vector<std::vector<std::vector<bool>>> visited(
        UPPER_LIMIT + 1, std::vector<std::vector<bool>>(UPPER_LIMIT + 1, std::vector<bool>(UPPER_LIMIT + 1, false)));
    std::queue<Point> bfs;
    bfs.push({0, 0, 0});
    while (!bfs.empty()) {
      auto front = bfs.front();
      bfs.pop();
      const auto& [x, y, z] = front;
      if (visited[x][y][z]) continue;
      visited[x][y][z] = true;
      exposedSides += countTouchingSides(front);
      for (const auto& direction : directions)
        if (isValidDestination(add(front, direction))) bfs.push(add(front, direction));
    }

    for (const auto& [x, y, z] : points)
      if (x == 0 || y == 0 || z == 0) exposedSides++;
    std::cout << "Part 2: " << exposedSides << '\n';
    return exposedSides;
  }

 private:
  using Point = std::tuple<int, int, int>;

  std::vector<Point> directions{{1, 0, 0}, {-1, 0, 0}, {0, 1, 0}, {0, -1, 0}, {0, 0, 1}, {0, 0, -1}};

  Point add(const Point& first, const Point& second) {
    return {std::get<0>(first) + std::get<0>(second), std::get<1>(first) + std::get<1>(second),
            std::get<2>(first) + std::get<2>(second)};
  }

  std::unordered_set<Point, PointHash> read(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    std::unordered_set<Point, PointHash> points;
    while (input >> line) {
      int x, y, z;
      std::sscanf(line.c_str(), "%i,%i,%i", &x, &y, &z);
      assert(x < 20 && y < 20 && z < 20);
      // assert(x > 0 && y > 0 && z > 0); // FAILS!
      points.insert({x, y, z});
    }
    return points;
  }
};

int main() {
  Solution s;
  s.solve("input");
  return 0;
}
