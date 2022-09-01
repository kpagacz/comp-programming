// link to the problem: https://leetcode.com/contest/weekly-contest-292/problems/largest-3-same-digit-number-in-string/
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

class Solution {
 public:
  std::string largestGoodInteger(std::string num) {
    for (char c = '9'; c >= '0'; --c) {
      std::string pattern(3, c);
      if (num.find(pattern) != std::string::npos) return pattern;
    }

    return "";
  }
};

int main(int argc, char** argv) {
  std::vector<std::string> tests;
  tests.push_back("6777133339");
  tests.push_back("2300019");
  tests.push_back("42352338");
  Solution s;
  for (auto& test : tests) std::cout << s.largestGoodInteger(test) << '\n';
}
