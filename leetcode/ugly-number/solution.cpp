// link to the problem: https://leetcode.com/problems/ugly-number/
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

// Runtime: 16 ms, faster than 5.21% of C++ online submissions for Ugly Number.
// Memory Usage: 6 MB, less than 20.95% of C++ online submissions for Ugly Number.

class Solution {
 public:
  bool isUgly(int n) {
    if (n <= 0) return false;
    std::vector<int> primeFactors{2, 3, 5};
    for (const auto& factor : primeFactors)
      while ((n % factor) == 0) n /= factor;
    return n == 1;
  }
};

int main(int argc, char** argv) { std::cout << (-15 % 2) << '\n'; }
