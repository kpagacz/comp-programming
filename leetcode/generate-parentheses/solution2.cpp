// link to the problem: https://leetcode.com/problems/generate-parentheses/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <vector>

class Solution {
 public:
  std::vector<std::string> generateParenthesis(int n) {
    std::vector<std::string> answer;
    if (n == 0) {
      answer.push_back("");
    } else {
      for (auto i{0}; i < n; i++) {
        for (const auto& left : generateParenthesis(i))
          for (const auto& right : generateParenthesis(n - 1 - i)) answer.push_back("(" + left + ")" + right);
      }
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  for (const auto& s : s.generateParenthesis(4)) {
    std::cout << s << " ";
  }
}
