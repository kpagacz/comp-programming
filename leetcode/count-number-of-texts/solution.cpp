// link to the problem: https://leetcode.com/contest/weekly-contest-292/problems/count-number-of-texts/
#include <math.h>

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

// Runtime: 31 ms, faster than 97.51% of C++ online submissions for Count Number of Texts.
// Memory Usage: 10.8 MB, less than 97.18% of C++ online submissions for Count Number of Texts.

constexpr int MODULO = 1e9 + 7;

class Solution {
 public:
  int countTexts(std::string pressedKeys) {
    int dp[5] = {1, 1, 1, 1, 1};
    for (int i = pressedKeys.size() - 1; i >= 0; --i) {
      dp[i % 5] = 0;
      int maxJ = std::min((int)pressedKeys.size(), i + (pressedKeys[i] == '7' || pressedKeys[i] == '9' ? 4 : 3));
      for (int j = i; j < maxJ && pressedKeys[i] == pressedKeys[j]; ++j)
        dp[i % 5] = (dp[i % 5] + dp[(j + 1) % 5]) % MODULO;
    }
    return dp[0];
  }
};

int main(int argc, char** argv) {}
