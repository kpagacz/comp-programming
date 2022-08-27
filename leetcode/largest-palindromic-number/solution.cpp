// link to the problem: https://leetcode.com/problems/largest-palindromic-number/
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

// Runtime: 32 ms, faster than 100.00% of C++ online submissions for Largest Palindromic Number.
// Memory Usage: 13 MB, less than 100.00% of C++ online submissions for Largest Palindromic Number.

class Solution {
 public:
  std::string largestPalindromic(std::string num) {
    std::vector<int> digits(10, 0);
    for (const auto& digit : num) digits[digit - '0']++;
    std::string answer;
    for (int i = 9; i >= 0; --i) {
      if (i == 0 && answer.empty()) continue;
      while (digits[i] > 1) {
        answer += '0' + i;
        digits[i] -= 2;
      }
    }
    bool odd = false;
    for (int i = 9; i >= 0; --i) {
      if (digits[i]) {
        answer += '0' + i;
        odd = true;
        break;
      }
    }
    int answerSize = answer.size();
    if (!odd) {
      answer += answer;
    } else {
      answer += answer.substr(0, answer.size() - 1);
    }
    std::reverse(answer.begin() + answerSize, answer.end());
    return answer;
  }
};

int main(int argc, char** argv) {}
