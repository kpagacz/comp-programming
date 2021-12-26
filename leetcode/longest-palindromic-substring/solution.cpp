// link to the problem
// https://leetcode.com/problems/longest-palindromic-substring/submissions/

// What have I learned from this problem?
// * how to insert a character between every character in a string:
// std::copy(s.begin(), s.end(), std::ostream_iterator<char>(os, "|"))
// * how to remove all occurences of a character from a string:
// str.erase(std::remove(begin(), end(), "|"),  str.end());
#include <iostream>
#include <iterator>
#include <sstream>
#include <string>

class Solution {
 public:
  std::string longestPalindrome(std::string s) {
    // prepare string
    std::ostringstream os;
    os << '|';
    std::copy(s.begin(), s.end(), std::ostream_iterator<char>(os, "|"));
    s = os.str();

    int longest_length = 0;
    int longest_start, longest_end;
    for (int i = 0; i < s.size(); i++) {
      int back = i, forward = i;
      while (back > 0 && forward < s.size() - 1) {
        if (s[back - 1] == s[forward + 1]) {
          back--;
          forward++;
        } else {
          break;
        }
      }
      if (forward - back > longest_length) {
        longest_length = forward - back;
        longest_start = back;
        longest_end = forward;
      }
    }

    auto answer = s.substr(longest_start, longest_length);
    answer.erase(std::remove(answer.begin(), answer.end(), '|'), answer.end());
    return answer;
  }
};
