// link to the problem: https://leetcode.com/problems/valid-palindrome/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Valid Palindrome.
// Memory Usage: 7.4 MB, less than 50.71% of C++ online submissions for Valid Palindrome.

class Solution {
 public:
  bool isPalindrome(std::string s) {
    s.erase(std::remove_if(s.begin(), s.end(), [](const auto x) { return !std::isalnum(x); }), s.end());
    std::transform(s.begin(), s.end(), s.begin(), [](const auto x) { return std::tolower(x); });
    auto forward = s.begin();
    auto reverse = s.rbegin();
    while (forward != s.end()) if (*forward++ != *reverse++) return false;
    return true;
  }
};

int main(int argc, char** argv) {}
