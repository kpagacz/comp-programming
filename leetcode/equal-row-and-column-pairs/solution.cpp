// link to the problem: https://leetcode.com/problems/equal-row-and-column-pairs/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

struct VectorHashFunction {
  template <typename T>
  std::size_t operator()(const std::vector<T>& v) const {
    std::hash<T> hashFunction;
    std::size_t answer = 0;
    for (const auto& element : v) answer ^= hashFunction(element) + 0x9e3779b9 + (answer << 6) + (answer >> 2);
    return answer;
  }
};

class Solution {
 public:
  int equalPairs(std::vector<std::vector<int>>& grid) {
    std::unordered_map<std::vector<int>, int, VectorHashFunction> columns, rows;
    for (const auto& row : grid) rows[row]++;
    for (int i = 0; i < grid[0].size(); i++) {
      std::vector<int> column(grid.size());
      for (int j = 0; j < grid.size(); ++j) column[j] = grid[j][i];
      columns[column]++;
    }

    int answer = 0;
    for (const auto& [row, count] : rows) {
      if (columns.count(row) > 0) answer += count * columns[row];
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  std::vector<std::vector<int>> grid{{3, 1, 2, 2}, {1, 4, 4, 4}, {2, 4, 2, 2}, {2, 5, 2, 2}};
  Solution s;
  std::cout << s.equalPairs(grid) << std::endl;
}
