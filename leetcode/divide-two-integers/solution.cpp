// link to the problem: https://leetcode.com/problems/divide-two-integers/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Divide Two Integers.
// Memory Usage: 5.9 MB, less than 28.39% of C++ online submissions for Divide Two Integers.

class Solution {
 public:
  int divide(int dividend, int divisor) {
    if (dividend == INT32_MIN && divisor == -1) return INT32_MAX;
    uint32_t a = std::abs(dividend);
    uint32_t b = std::abs(divisor);
    uint32_t res = 0;
    for (int x = 31; x >= 0; --x) {
      if ((a >> x) >= b) {
        res += 1 << x;
        a -= b << x;
      }
    }

    return (dividend < 0) ^ (divisor < 0) ? -res : res;
  }
};

int main(int argc, char** argv) {}
