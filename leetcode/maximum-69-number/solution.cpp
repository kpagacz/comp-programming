// link to the problem: https://leetcode.com/problems/maximum-69-number/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Maximum 69 Number.
// Memory Usage: 5.9 MB, less than 45.85% of C++ online submissions for Maximum 69 Number.

class Solution {
 public:
  int maximum69Number(int num) {
    std::string number = std::to_string(num);
    std::size_t found = number.find_first_not_of('9');
    if (found == std::string::npos) return num;
    number[found] = '9';
    return std::stoi(number);
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << s.maximum69Number(9669) << '\n';
  std::cout << s.maximum69Number(9999) << '\n';
}
