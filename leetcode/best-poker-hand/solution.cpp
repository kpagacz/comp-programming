// link to the problem: https://leetcode.com/problems/best-poker-hand/
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
  std::string bestHand(std::vector<int>& ranks, std::vector<char>& suits) {
    std::sort(suits.begin(), suits.end());
    if (std::unique(suits.begin(), suits.end()) - suits.begin() == 1) return "Flush";
    std::vector<int> ranksCount(14, 0);
    for (const auto& rank : ranks) ranksCount[rank]++;
    std::sort(ranksCount.begin(), ranksCount.end(), std::greater<int>());
    switch (ranksCount[0]) {
      case 1:
        return "High Card";
      case 2:
        return "Pair";
      case 3:
        return "Three of a Kind";
      default:
        return "Three of a Kind";
    }

    return "Error";
  }
};

int main(int argc, char** argv) {}
