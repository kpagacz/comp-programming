// link to the problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

class Solution {
 public:
  int lengthOfLongestSubstring(std::string s) {
    std::vector<int> chars(256, -1);
    int max_substring = 0, curr_substring = 0;
    for (auto i{0}; i < s.size(); i++) {
      std::cout << "char: " << s[i] << " curr: " << curr_substring << '\n';
      if (chars[s[i]] == -1) {
        chars[s[i]] = i;
        curr_substring++;
      } else {
        auto dup = chars[s[i]];
        std::fill(chars.begin(), chars.end(), -1);
        max_substring = std::max(max_substring, curr_substring);
        for (auto j{dup + 1}; j <= i; j++) {
          chars[s[j]] = j;
        }
        curr_substring = i - dup;
      }
    }
    max_substring = std::max(max_substring, curr_substring);
    return max_substring;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << s.lengthOfLongestSubstring("pwwkew");
}
