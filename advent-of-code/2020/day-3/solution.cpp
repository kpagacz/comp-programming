#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

using NumType = uint64_t;
class Solution {
 public:
  NumType part1(const std::string& path) {
    auto grid = parseInput(path);
    return encounteredTrees(grid, {3, 1});
  }

  NumType part2(const std::string& path) {
    auto grid = parseInput(path);
    std::vector<std::pair<NumType, NumType>> slopes{{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}};
    return std::transform_reduce(slopes.begin(), slopes.end(), 1ull, std::multiplies<NumType>(),
                                 [&](const auto& slope) { return encounteredTrees(grid, slope); });
  }

 private:
  using Grid = std::vector<std::vector<char>>;
  Grid parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string row;
    Grid grid;
    while (input >> row) grid.push_back(std::vector<char>(row.begin(), row.end()));

    return grid;
  }

  NumType encounteredTrees(const Grid& grid, const std::pair<NumType, NumType>& slope) {
    NumType trees = 0;
    NumType col = 0, row = 0;
    while (row < grid.size()) {
      if (grid[row][col % grid[0].size()] == '#') trees++;
      row += slope.second, col += slope.first;
    }
    return trees;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
