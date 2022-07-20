// link to the problem: https://leetcode.com/problems/fibonacci-number/
#include <algorithm>
#include <array>
#include <cassert>
#include <iomanip>
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Fibonacci Number.
// Memory Usage: 6.1 MB, less than 9.61% of C++ online submissions for Fibonacci Number.

class Solution {
 public:
  int fib(int n) {
    double binet = 1.0 / std::sqrt(5) * std::pow((1 + std::sqrt(5)) / 2, n);
    return std::round(binet);
  }
};

int main(int argc, char** argv) {}
