// link to the problem: https://leetcode.com/problems/sqrtx/
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

// Aweful runtime because it needs to be smarter than this (binary search maybe?)
// Runtime: 19 ms, faster than 16.29% of C++ online submissions for Sqrt(x).
// Memory Usage: 5.9 MB, less than 73.18% of C++ online submissions for Sqrt(x).

class Solution {
 public:
  int mySqrt(int x) {
    int previousInt{0};
    for (int64_t i{1}; i < INT_MAX; ++i) {
      if (i * i > x) {
        return previousInt;
      } else
        previousInt = i;
    }
    return -1;
  }
};

int main(int argc, char** argv) {}
