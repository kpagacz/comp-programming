// link to the problem: https://leetcode.com/problems/count-and-say/
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

// Runtime: 16 ms, faster than 52.36% of C++ online submissions for Count and Say.
// Memory Usage: 6.7 MB, less than 56.81% of C++ online submissions for Count and Say.

class Solution {
 public:
  std::string countAndSay(int n) {
    std::string number = "1";
    for (int i = 1; i < n; i++) {
      std::string newNumber = "";
      std::size_t currentChar = 0;
      std::size_t nextChar = number.find_first_not_of(number[currentChar], currentChar);
      while (nextChar != std::string::npos) {
        newNumber += std::to_string(nextChar - currentChar);
        newNumber += number[currentChar];
        currentChar = nextChar;
        nextChar = number.find_first_not_of(number[currentChar], currentChar);
      }
      newNumber += std::to_string(number.length() - currentChar);
      newNumber += number[currentChar];
      number = newNumber;
    }
    return number;
  }
};

int main(int argc, char** argv) {
  Solution s;
  for (auto& i : {1, 2, 3, 4, 5, 30}) std::cout << s.countAndSay(i) << '\n';
}
