// link to the problem: https://leetcode.com/problems/excel-sheet-column-number/
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
  int titleToNumber(std::string columnTitle) {
    return std::accumulate(columnTitle.begin(), columnTitle.end(), 0,
                           [](const auto& sum, const auto& c) { return sum * 26 + c - 'A' + 1; });
  }
};

int main(int argc, char** argv) {}
