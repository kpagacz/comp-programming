// link to the problem: https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up/
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

// Runtime: 714 ms, faster than 28.94% of C++ online submissions for Minimum Consecutive Cards to Pick Up.
// Memory Usage: 115.2 MB, less than 49.54% of C++ online submissions for Minimum Consecutive Cards to Pick Up.

class Solution {
 public:
  int minimumCardPickup(std::vector<int>& cards) {
    std::unordered_map<int, int> lastIndexes;

    int answer = INT32_MAX;
    for (int i = 0; i < cards.size(); ++i) {
      const int& card = cards[i];
      if (lastIndexes.count(card) == 0)
        lastIndexes[card] = i;
      else {
        answer = std::min(answer, i - lastIndexes[card]);
        lastIndexes[card] = i;
      }
    }

    return answer == INT32_MAX ? -1 : answer;
  }
};

int main(int argc, char** argv) {}
