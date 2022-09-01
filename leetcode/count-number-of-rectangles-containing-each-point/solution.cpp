// link to the problem: https://leetcode.com/problems/count-number-of-rectangles-containing-each-point/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <map>
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

// Runtime: 785 ms, faster than 77.52% of C++ online submissions for Count Number of Rectangles Containing Each Point.
// Memory Usage: 84.4 MB, less than 81.71% of C++ online submissions for Count Number of Rectangles Containing Each
// Point.

class Solution {
 public:
  std::vector<int> countRectangles(std::vector<std::vector<int>>& rectangles, std::vector<std::vector<int>>& points) {
    std::map<int, std::vector<int>> rectanglesMap;
    for (const auto& rectangle : rectangles) rectanglesMap[rectangle[1]].push_back(rectangle[0]);
    for (auto& [x, ys] : rectanglesMap) std::sort(ys.begin(), ys.end());

    std::vector<int> answer(points.size());
    for (int i = 0; i < points.size(); ++i) {
      const auto& point = points[i];
      auto xLowerBound = rectanglesMap.lower_bound(point[1]);
      int containingRectangles = 0;
      while (xLowerBound != rectanglesMap.end()) {
        containingRectangles += xLowerBound->second.end() -
                                std::lower_bound(xLowerBound->second.begin(), xLowerBound->second.end(), point[0]);
        ++xLowerBound;
      }
      answer[i] = containingRectangles;
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<std::vector<std::vector<int>>, std::vector<std::vector<int>>>> tests;
  tests.push_back({{{1, 2}, {2, 3}, {2, 5}}, {{2, 1}, {1, 4}}});
  for (auto& [rectangles, points] : tests) {
    auto result = s.countRectangles(rectangles, points);
    std::copy(result.begin(), result.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
  }
}
