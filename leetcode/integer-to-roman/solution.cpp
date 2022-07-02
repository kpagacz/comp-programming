// link to the problem: https://leetcode.com/problems/integer-to-roman/
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
  std::string intToRoman(int num) {
    std::string roman;
    auto romanMapIt = romanMap.rbegin();
    while (num > 0) {
      const auto& [value, romanRepresentation] = *romanMapIt;
      if (value > num)
        ++romanMapIt;
      else {
        roman += romanRepresentation;
        num -= value;
      }
    }

    return roman;
  }

 private:
  std::vector<std::pair<int, std::string>> romanMap = {{1, "I"},   {4, "IV"},   {5, "V"},   {9, "IX"},  {10, "X"},
                                                       {40, "XL"}, {50, "L"},   {90, "XC"}, {100, "C"}, {400, "CD"},
                                                       {500, "D"}, {900, "CM"}, {1000, "M"}};
};

int main(int argc, char** argv) {}
