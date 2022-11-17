// link to the problem: https://leetcode.com/problems/rectangle-area/
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

// Runtime: 24 ms, faster than 18.00% of C++ online submissions for Rectangle Area.
// Memory Usage: 6 MB, less than 32.00% of C++ online submissions for Rectangle Area.

class Solution {
 public:
  int computeArea(int ax1, int ay1, int ax2, int ay2, int bx1, int by1, int bx2, int by2) {
    int sumArea = (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1);

    // intersection
    int x1 = std::max(ax1, bx1);
    int y1 = std::max(ay1, by1);
    int x2 = std::min(ax2, bx2);
    int y2 = std::min(ay2, by2);

    int intersection = 0;
    if (x1 < x2 && y1 < y2) intersection = (x2 - x1) * (y2 - y1);
    return sumArea - intersection;
  }
};

int main(int argc, char** argv) {}
