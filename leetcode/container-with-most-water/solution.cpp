// link to the problem: https://leetcode.com/problems/container-with-most-water/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

class Solution {
 public:
  int maxArea(std::vector<int>& height) {
    int left = 0, right = height.size() - 1;
    int max_area = 0;
    while (left < right) {
      max_area = std::max(max_area, (right - left) * std::min(height[left], height[right]));
      auto from_right = height.size() - 1 - right;
      if (height[left] == height[right]) {
        auto found_left =
            std::find_if(height.begin() + left, height.end(), [&](const auto& a) { return a > height[left]; });
        auto found_right =
            std::find_if(height.rbegin() + from_right, height.rend(), [&](const auto& a) { return a > height[right]; });
        if (found_left == height.end()) break;
        if (std::distance(height.begin() + left, found_left) >
            std::distance(height.rbegin() + from_right, found_right)) {
          right -= std::distance(height.rbegin() + from_right, found_right);
        } else {
          left += std::distance(height.begin() + left, found_left);
        }
        continue;
      }
      if (height[left] < height[right]) {
        left = std::distance(height.begin(), std::find_if(height.begin() + left, height.end(),
                                                          [&](const auto& a) { return a > height[left]; }));
      } else {right -= std::distance(
          height.rbegin() + from_right,
          std::find_if(height.rbegin() + from_right, height.rend(), [&](const auto& a) { return a > height[right]; }));
      }
    }
    return max_area;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> heights{1, 1000, 1000, 6, 2, 5, 4, 8, 3, 7};
  std::cout << s.maxArea(heights);
}
