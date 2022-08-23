// link to the problem: https://leetcode.com/problems/power-of-four/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Power of Four.
// Memory Usage: 5.7 MB, less than 88.37% of C++ online submissions for Power of Four.

class Solution {
 public:
  bool isPowerOfFour(int n) {
    std::bitset<32> num(n);
    n = 0;
    for (int i = 0; i < 32 && !num[i]; ++i) n++;
    return num.count() == 1 && (n % 2 == 0);
  }
};

int main(int argc, char** argv) {}
