// link to the problem: https://leetcode.com/problems/largest-rectangle-in-histogram/
// What I have learned - Monotonic stack
// Monotonic stack is useful to solve problems exhibiting the three characteristics:
// 1. it is a "range queries in an array" problem
// 2. The minima/maxima elment or the monotonic order of elements in a range
//    is useful to get answer of every range query
// 3. When an elements is popped from the monotnic stack, it will never be used again

#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
public:
    int largestRectangleArea(std::vector<int>& heights) {
      std::stack<int> mono_stack;
      heights.push_back(0);
      int answer = 0;
      for (auto i{0}; i < heights.size(); i++) {
        while (!mono_stack.empty() && heights[mono_stack.top()] > heights[i]) {
          int height = heights[mono_stack.top()];
          mono_stack.pop();
          int length;
          if (!mono_stack.empty())
            length = i - mono_stack.top() - 1;
            else
            length = i;
            answer = std::max(answer, height * length);
        }
        mono_stack.push(i);
      }
      return answer;
    }
};

int main(int argc, char** argv) {}
