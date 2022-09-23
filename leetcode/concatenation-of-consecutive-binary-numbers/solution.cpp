// link to the problem: https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/
#include <algorithm>
#include <array>
#include <bitset>
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

// Runtime: 146 ms, faster than 48.37% of C++ online submissions for Concatenation of Consecutive Binary Numbers.
// Memory Usage: 5.9 MB, less than 82.61% of C++ online submissions for Concatenation of Consecutive Binary Numbers.

constexpr int MOD = 1e9 + 7;

class Solution {
 public:
  int concatenatedBinary(int n) {
    int64_t answer = 0;
    for (int i = 1; i <= n; ++i) {
      int shift = log2(i) + 1;
      answer = ((answer << shift) % MOD + i) % MOD;
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
