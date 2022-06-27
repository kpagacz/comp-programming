// link to the problem: https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/
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

// Probably nicer to look at, but slower overall
// Runtime: 89 ms, faster than 47.45% of C++ online submissions for Maximum Points You Can Obtain from Cards.
// Memory Usage: 44.1 MB, less than 16.19% of C++ online submissions for Maximum Points You Can Obtain from Cards.

class Solution {
 public:
  int maxScore(std::vector<int>& cardPoints, int k) {
    std::vector<int> partialSums(cardPoints.size() + 1);
    partialSums[0] = 0;
    std::partial_sum(cardPoints.begin(), cardPoints.end(), partialSums.begin() + 1);
    int totalPoints = std::accumulate(cardPoints.begin(), cardPoints.end(), 0);
    int minLeft = totalPoints;
    int leftCards = cardPoints.size() - k;
    for (int i{leftCards}; i < partialSums.size(); ++i)
      minLeft = std::min(minLeft, partialSums[i] - partialSums[i - leftCards]);
    return totalPoints - minLeft;
  }
};

int main(int argc, char** argv) {}
