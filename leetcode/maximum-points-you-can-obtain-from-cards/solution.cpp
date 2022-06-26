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

class Solution {
 public:
  int maxScore(std::vector<int>& cardPoints, int k) {
    if (k >= cardPoints.size()) return std::accumulate(cardPoints.begin(), cardPoints.end(), 0);
    int cardsToLeft = cardPoints.size() - k;
    int sum = std::accumulate(cardPoints.begin(), cardPoints.end(), 0);
    int minLeft = sum;
    std::size_t left = 0, right = cardsToLeft;
    int currentSum = std::accumulate(cardPoints.begin(), cardPoints.begin() + right, 0);
    minLeft = std::min(currentSum, minLeft);
    while(right < cardPoints.size()) {
      currentSum -= cardPoints[left++];
      currentSum += cardPoints[right++];
      minLeft = std::min(currentSum, minLeft);
    }
    return sum - minLeft;
  }
};

int main(int argc, char** argv) {}
