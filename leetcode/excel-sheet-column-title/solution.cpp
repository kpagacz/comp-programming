// link to the problem: https://leetcode.com/problems/excel-sheet-column-title/
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
  std::string convertToTitle(int columnNumber) {
    std::string result = "";
    while (columnNumber > 0) {
      result += static_cast<char>((columnNumber - 1) % 26 + 'A');
      columnNumber = (columnNumber - 1) / 26;
    }
    std::reverse(result.begin(), result.end());
    return result;
  }
};

int main(int argc, char** argv) {}
