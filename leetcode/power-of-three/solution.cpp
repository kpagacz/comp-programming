// link to the problem: https://leetcode.com/problems/power-of-three/submissions/
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

// Runtime: 12 ms, faster than 97.82% of C++ online submissions for Power of Three.
// Memory Usage: 5.9 MB, less than 72.69% of C++ online submissions for Power of Three.

constexpr int LARGEST_POWER_OF_3 = 1162261467;

class Solution {
 public:
  bool isPowerOfThree(int n) { return n > 0 && LARGEST_POWER_OF_3 % n == 0; }
};

int main(int argc, char** argv) {}
