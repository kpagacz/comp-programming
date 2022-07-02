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

// I have a bug here that happens for large integers. Test case that fails:
// 1000000000
// 1000000000
// [1000000]
// [1000000]
// Expected is: 13993049
// My outputs: 19991100
// Found the bug. Turns out:
// I did not sort cuts when I should.
// adjacent_differences returns the difference from the first element to the zero indexed element, so I did
// not need to push it on my own.

// Nonetheless, this gets TLE.

constexpr int64_t MOD = 1e9 + 7;

class Solution {
 public:
  int maxArea(int h, int w, std::vector<int>& horizontalCuts, std::vector<int>& verticalCuts) {
    std::vector<int> horizontalDifferences, verticalDifferences;
    std::sort(horizontalCuts.begin(), horizontalCuts.end());
    std::sort(verticalCuts.begin(), verticalCuts.end());
    std::adjacent_difference(horizontalCuts.begin(), horizontalCuts.end(), std::back_inserter(horizontalDifferences));
    std::adjacent_difference(verticalCuts.begin(), verticalCuts.end(), std::back_inserter(verticalDifferences));
    horizontalDifferences.push_back(h - horizontalCuts.back());
    verticalDifferences.push_back(w - verticalCuts.back());


    int64_t answer = 0;
    for (const auto& vDiff : verticalDifferences)
      for (const auto& hDiff : horizontalDifferences) answer = std::max(answer, (int64_t)vDiff * hDiff);
    return answer % MOD;
  }
};

int main(int argc, char** argv) {}
