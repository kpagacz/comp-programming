// link to the problem: https://leetcode.com/problems/remove-digit-from-number-to-maximize-result/
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

// Runtime: 3 ms, faster than 61.24% of C++ online submissions for Remove Digit From Number to Maximize Result.
// Memory Usage: 7 MB, less than 23.25% of C++ online submissions for Remove Digit From Number to Maximize Result.

class Solution {
 public:
  std::string removeDigit(std::string number, char digit) {
    std::string answer;
    std::size_t nextPos = number.find(digit, 0);
    while (nextPos != std::string::npos) {
      std::string newNumber = number.substr(0, nextPos) + number.substr(nextPos + 1);
      if (newNumber > answer) answer = newNumber;
      nextPos = number.find(digit, nextPos + 1);
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
