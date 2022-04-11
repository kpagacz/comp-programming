// link to the problem: https://leetcode.com/problems/remove-k-digits/
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
  std::string removeKdigits(std::string num, int k) {
    if (k == num.size()) return "0";

    std::stack<char> digits;
    for(const auto& c : num) {
      while(k && !digits.empty() && digits.top() > c) {
        digits.pop();
        k--;
      }
      digits.push(c);
    }

    while(k--) {
      digits.pop();
    }

    std::string answer;
    while(!digits.empty()) {
      answer += digits.top();
      digits.pop();
    }
    while (!answer.empty() && answer.back() == '0') answer.pop_back();
    std::reverse(answer.begin(), answer.end());
    if (answer.empty()) return "0";
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << s.removeKdigits("1234", 2) << '\n';
  std::cout << s.removeKdigits("9178", 2) << '\n';
  std::cout << s.removeKdigits("9871", 2) << '\n';
  std::cout << s.removeKdigits("1432219", 3) << '\n';
 }
