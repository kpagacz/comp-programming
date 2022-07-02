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

const int MOD = 1e9 + 7;

class Solution {
 public:
  int maxArea(int h, int w, std::vector<int>& horizontalCuts, std::vector<int>& verticalCuts) {
    long int maxHeight = getMaxGap(horizontalCuts, h), maxWidth = getMaxGap(verticalCuts, w);

    return (maxWidth * maxHeight) % MOD;
  }

  int getMaxGap(const std::vector<int>& cuts, const int size) {
    int minCut = *std::min_element(begin(cuts), end(cuts)), maxCut = *max_element(begin(cuts), end(cuts));
    int bucketSize = std::max(1, (maxCut - minCut) / (int)cuts.size()), prevBucketMax = minCut,
        ans = std::max(minCut, size - maxCut);
    std::vector<std::tuple<bool, int, int>> buckets((maxCut - minCut) / bucketSize + 1, {false, INT_MAX, INT_MIN});

    for (const int& cut : cuts) {
      int bucketIdx = (cut - minCut) / bucketSize;

      std::get<0>(buckets[bucketIdx]) = true;
      std::get<1>(buckets[bucketIdx]) = std::min(get<1>(buckets[bucketIdx]), cut);
      std::get<2>(buckets[bucketIdx]) = std::max(get<2>(buckets[bucketIdx]), cut);
    }

    // print buckets
    for (const auto& [used, bucketMin, bucketMax] : buckets) {
      if (used) std::cout << bucketMin << " " << bucketMax << '\n';
    }

    for (auto& [used, bucketMin, bucketMax] : buckets) {
      if (not used) {
        continue;
      }

      ans = std::max(ans, bucketMin - prevBucketMax);
      prevBucketMax = bucketMax;
    }

    return ans;
  }
};
