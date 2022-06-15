// link to the problem: https://leetcode.com/problems/triangle/
#include <algorithm>
#include <array>
#include <cassert>
#include <cstring>
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

class Solution {
 public:
  int minimumTotal(std::vector<std::vector<int>>& triangle) {
    int min_path = triangle[0][0];
    std::vector<int> paths_on_level(triangle.size(), INT32_MAX);
    paths_on_level[0] = triangle[0][0];
    std::vector<int> paths_on_next_level(triangle.size(), INT32_MAX);
    for (int level{0}; level < triangle.size() - 1; level++) {
      for (int el{0}; el < triangle[level].size(); el++) {
        paths_on_next_level[el] = std::min(paths_on_next_level[el], paths_on_level[el] + triangle[level + 1][el]);
        paths_on_next_level[el + 1] =
            std::min(paths_on_next_level[el + 1], paths_on_level[el] + triangle[level + 1][el + 1]);
      }
      auto min_element = std::min_element(paths_on_next_level.begin(), paths_on_next_level.end());
      min_path = *min_element;
      paths_on_level = paths_on_next_level;
      std::fill(paths_on_next_level.begin(), paths_on_next_level.end(), INT32_MAX);
    }
    return min_path;
  }
};

int main() {
  Solution s;
  std::vector<std::vector<int>> v{{2}, {3, 4}, {6, 5, 7}, {4, 1, 8, 3}};
  // std::vector<std::vector<int>> v{{-10}};
  std::cout << s.minimumTotal(v);
}
