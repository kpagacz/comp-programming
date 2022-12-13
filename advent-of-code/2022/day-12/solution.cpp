#include <fstream>
#include <iostream>
#include <queue>
#include <string>
#include <vector>

class Solution {
 public:
  int part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    auto grid = readGrid(input);
    auto [start, end] = getStartAndEnd(grid);
    grid[start.first][start.second] = 'a', grid[end.first][end.second] = 'z';

    // std::cout << "\nStart is: " << start.first << " " << start.second << '\n';
    // std::cout << "End is: " << end.first << " " << end.second << '\n';

    std::queue<point> points;
    points.push(start);
    int steps = -1;
    while (!points.empty()) {
      steps++;
      std::queue<point> newPoints;
      while (!points.empty()) {
        auto [row, col] = points.front();
        // std::cout << "Point: " << row << " " << col << '\n';
        points.pop();
        if (row == end.first && col == end.second) return steps;
        if (grid[row][col] == VISITED) continue;
        for (auto [deltaRow, deltaCol] : directions)
          if (isValidDestination({row, col}, {row + deltaRow, col + deltaCol}, grid))
            newPoints.push({row + deltaRow, col + deltaCol});

        grid[row][col] = VISITED;
      }
      points = newPoints;
    }
    return -1;
  }

  int part2(const std::string& pathToInput) {
       std::fstream input(pathToInput, std::ios_base::in);
    auto grid = readGrid(input);
    auto [start, end] = getStartAndEnd(grid);
    grid[start.first][start.second] = 'a', grid[end.first][end.second] = 'z';

    // std::cout << "\nStart is: " << start.first << " " << start.second << '\n';
    // std::cout << "End is: " << end.first << " " << end.second << '\n';

    std::queue<point> points;
    points.push(end);
    int steps = -1;
    while (!points.empty()) {
      steps++;
      std::queue<point> newPoints;
      while (!points.empty()) {
        auto [row, col] = points.front();
        // std::cout << "Point: " << row << " " << col << '\n';
        points.pop();
        if (grid[row][col] == 'a') return steps;
        if (grid[row][col] == 'X') continue;
        for (auto [deltaRow, deltaCol] : directions)
          if (isValidDestinationPart2({row, col}, {row + deltaRow, col + deltaCol}, grid))
            newPoints.push({row + deltaRow, col + deltaCol});

        grid[row][col] = VISITED;
      }
      points = newPoints;
    }
    return -1;
  }

 private:
  using grid = std::vector<std::vector<char>>;
  using point = std::pair<int, int>;
  const std::vector<std::pair<int, int>> directions{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
  const char VISITED = 'X';
  bool isValidDestination(const point& source, const point& destination, const grid& grid) {
    return destination.first >= 0 && destination.first < grid.size() && destination.second >= 0 &&
           destination.second < grid[0].size() && grid[destination.first][destination.second] != VISITED &&
           grid[destination.first][destination.second] <= grid[source.first][source.second] + 1;
  }

  bool isValidDestinationPart2(const point& source, const point& destination, const grid& grid) {
return destination.first >= 0 && destination.first < grid.size() && destination.second >= 0 &&
           destination.second < grid[0].size() && grid[destination.first][destination.second] != VISITED &&
           grid[destination.first][destination.second] + 1 >= grid[source.first][source.second];
  }
  grid readGrid(std::fstream& input) {
    grid grid;
    std::string line;
    while (input >> line) {
      grid.push_back(std::vector<char>());
      for (const auto& height : line) grid.back().push_back(height);
    }
    return grid;
  }

  std::pair<point, point> getStartAndEnd(const grid& grid) {
    point start, end;
    for (auto row{0}; row < grid.size(); row++)
      for (auto col{0}; col < grid[0].size(); col++) switch (grid[row][col]) {
          case 'S':
            start = {row, col};
            break;
          case 'E':
            end = {row, col};
            break;
          default:
            continue;
        }

    return {start, end};
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return -1;
}
