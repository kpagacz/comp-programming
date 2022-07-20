// link to the problem:
// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
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

constexpr int64_t MOD = 1e9 + 7;

class Solution {
 public:
  int maxArea(int h, int w, std::vector<int>& horizontalCuts, std::vector<int>& verticalCuts) {
    horizontalCuts.push_back(h);
    verticalCuts.push_back(w);
    std::sort(horizontalCuts.begin(), horizontalCuts.end());
    std::sort(verticalCuts.begin(), verticalCuts.end());
    std::adjacent_difference(horizontalCuts.begin(), horizontalCuts.end(), horizontalCuts.begin());
    std::adjacent_difference(verticalCuts.begin(), verticalCuts.end(), verticalCuts.begin());
    return ((int64_t)*std::max_element(horizontalCuts.begin(), horizontalCuts.end()) *
            (int64_t)*std::max_element(verticalCuts.begin(), verticalCuts.end())) %
           MOD;
  }
};

int main(int argc, char** argv) {}
