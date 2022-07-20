// link to the problem: https://leetcode.com/problems/trapping-rain-water/
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

// Runtime: 99 ms, faster than 5.32% of C++ online submissions for Trapping Rain Water.
// Memory Usage: 67.4 MB, less than 5.19% of C++ online submissions for Trapping Rain Water.

class Solution {
 public:
  int trap(std::vector<int>& height) {
    std::stack<int> monotonicStack;
    auto heightFirstMax = height.begin();
    while (heightFirstMax + 1 != height.end() && *heightFirstMax < *(heightFirstMax + 1)) ++heightFirstMax;
    if (heightFirstMax == height.end()) return 0;
    int first = std::distance(height.begin(), heightFirstMax);

    int totalWater = 0;
    for (int i = first; i < height.size(); ++i) {
      std::queue<int> underWater;
      while (!monotonicStack.empty() && height[i] > height[monotonicStack.top()]) {
        underWater.push(monotonicStack.top());
        monotonicStack.pop();
      }
      if (!monotonicStack.empty()) underWater.push(monotonicStack.top());
      monotonicStack.push(i);
      if (underWater.empty()) continue;

      int waterLevel = std::min(height[i], height[underWater.back()]);
      while (underWater.size() > 1) {
        const auto& curr = underWater.front();
        underWater.pop();
        const auto& next = underWater.front();
        totalWater += (waterLevel - height[curr]) * (curr - next);
      }
    }
    return totalWater;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> heights{4, 2, 0, 3, 2, 5};
  std::cout << s.trap(heights) << '\n';
}
