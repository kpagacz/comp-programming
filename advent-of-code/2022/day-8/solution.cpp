#include <assert.h>

#include <algorithm>
#include <array>
#include <fstream>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

class Solution {
 public:
  uint64_t part1(const std::string& pathToInput) {
    auto grid = readInput(pathToInput);
    auto visibleIndicator = std::vector<std::vector<char>>(grid.size(), std::vector<char>(grid[0].size(), 'n'));
    // std::cout << "Grid size: " << grid.size() << " x " << grid[0].size() << '\n';
    assert(grid.size() == visibleIndicator.size());
    assert(grid[0].size() == visibleIndicator[0].size());

    for (int row = 0; row < grid.size(); row++) {
      char previous = '/';
      for (int col = 0; col < grid[0].size(); col++)
        if (grid[row][col] > previous) visibleIndicator[row][col] = 'v', previous = grid[row][col];
      previous = '/';
      for (int col = grid[0].size() - 1; col >= 0; col--)
        if (grid[row][col] > previous) visibleIndicator[row][col] = 'v', previous = grid[row][col];
    }

    for (int col = 0; col < grid[0].size(); col++) {
      char previous = '/';
      for (int row = 0; row < grid.size(); row++)
        if (grid[row][col] > previous) visibleIndicator[row][col] = 'v', previous = grid[row][col];
      previous = '/';
      for (int row = grid.size() - 1; row >= 0; row--)
        if (grid[row][col] > previous) visibleIndicator[row][col] = 'v', previous = grid[row][col];
    }

    uint64_t visibleTrees = 0;
    for (const auto& line : visibleIndicator)
      for (const auto& indicator : line)
        if (indicator == 'v') ++visibleTrees;
    return visibleTrees;
  }

  uint64_t part2(const std::string& pathToInput) {
    auto grid = readInput(pathToInput);

    auto score = [&](std::vector<char> trees) {
      std::vector<int> scores(trees.size());
      trees.push_back('9' + 1);
      std::vector<uint64_t> decreasingStack;
      for (int i = 0; i < trees.size(); ++i) {
        while (!decreasingStack.empty() && trees[decreasingStack.back()] < trees[i]) {
          auto top = decreasingStack.back();
          decreasingStack.pop_back();
          uint64_t lastGreaterEqual;
          if (decreasingStack.empty()) lastGreaterEqual = 0;
          else lastGreaterEqual = decreasingStack.back();
          scores[top] = top - lastGreaterEqual;
        }
        decreasingStack.push_back(i);
      }

      assert(decreasingStack.size() == 1 && decreasingStack.back() == trees.size() - 1);
      return scores;
    };

    std::vector<std::vector<uint64_t>> scores(grid.size(), std::vector<uint64_t>(grid[0].size(), 1));

    for (int rowIndex = 0; rowIndex < grid.size(); rowIndex++) {
      auto row = grid[rowIndex];
      auto leftScores = score(row);
      std::reverse(row.begin(), row.end());
      auto rightScores = score(row);
      std::reverse(rightScores.begin(), rightScores.end());

      for (int col = 0; col < row.size(); col++) {
        scores[rowIndex][col] *= leftScores[col];
        scores[rowIndex][col] *= rightScores[col];
      }
    }
    for (int colIndex = 0; colIndex < grid[0].size(); colIndex++) {
      auto col = std::vector<char>();
      for (auto row = 0; row < grid.size(); row++) col.push_back(grid[row][colIndex]);

      auto upScores = score(col);
      std::reverse(col.begin(), col.end());
      auto downScores = score(col);
      std::reverse(downScores.begin(), downScores.end());

      for (int row = 0; row < col.size(); row++) {
        scores[row][colIndex] *= upScores[row];
        scores[row][colIndex] *= downScores[row];
      }
    }

    uint64_t max = 0;
    for (const auto& scoreRow : scores)
      for (const auto& score : scoreRow) max = std::max(max, score);
    return max;
  }

 private:
  const std::array<std::pair<int, int>, 4> directions{{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}};
  std::vector<std::vector<char>> readInput(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string line;
    std::vector<std::vector<char>> grid;
    while (input >> line) {
      grid.push_back(std::vector<char>());
      grid.back().reserve(line.size());
      for (auto c : line) grid.back().push_back(c);
    }
    return grid;
  }
};
int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
