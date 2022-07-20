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

// This one was actually slowe...

class Solution {
 public:
  int trap(std::vector<int>& height) {
    std::stack<int> monotonicStack;
    int totalWater = 0;
    for (int i = 0; i < height.size(); ++i) {
      std::queue<int> underWater;
      int curr;
      bool wasEmpty = monotonicStack.empty();
      while (!monotonicStack.empty() && height[i] > height[monotonicStack.top()]) {
        curr = monotonicStack.top();
        monotonicStack.pop();
        if (!monotonicStack.empty()) totalWater += (height[i] - height[curr]) * (curr - monotonicStack.top());
      }
      if (monotonicStack.empty() && !wasEmpty) totalWater -= (height[i] - height[curr]) * (i - curr - 1);
      monotonicStack.push(i);
    }
    return totalWater;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> heights{4, 2, 0, 3, 2, 5};
  std::vector<std::vector<int>> cases{{0, 2, 0}, {0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}, {4, 2, 0, 3, 2, 5}};
  for (auto& v : cases) std::cout << s.trap(v) << '\n';
}
