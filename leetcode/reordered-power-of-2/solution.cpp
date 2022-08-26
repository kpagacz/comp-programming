// link to the problem:
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Reordered Power of 2.
// Memory Usage: 7 MB, less than 14.97% of C++ online submissions for Reordered Power of 2.

class Solution {
 public:
  bool reorderedPowerOf2(int n) {
    std::vector<std::vector<int>> powerOf2Digits;
    for (int i = 0; i <= 29; i++) {
      std::vector<int> digits(10, 0);
      int power = 1 << i;
      std::string powerString = std::to_string(power);
      for (const auto& digit : powerString) digits[digit - '0']++;
      powerOf2Digits.push_back(digits);
    }

    std::vector<int> digitsOfN(10, 0);
    std::string nString = std::to_string(n);
    for (const auto& digit : nString) digitsOfN[digit - '0']++;

    return std::any_of(powerOf2Digits.begin(), powerOf2Digits.end(),
                       [&](const auto& digitsPattern) { return digitsOfN == digitsPattern; });
  }
};

int main(int argc, char** argv) {}
