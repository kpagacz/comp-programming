// link to the problem: https://leetcode.com/problems/zigzag-conversion/
#include <iostream>
#include <string>

class Solution {
 public:
  std::string convert(std::string s, int numRows) {
    if (numRows == 1) return s;
    std::string answer;
    for (int i = 0; i < s.size(); i += 2 * (numRows - 1)) {
      answer += s[i];
    }

    for (int i = 1; i < numRows - 1; i++) {
      for (int j = 0; j < s.size(); j += 2 * (numRows - 1)) {
        if (j + i < s.size()) answer += s[j + i];
        if (j + 2 * (numRows - 1) - i < s.size()) answer += s[j + 2 * (numRows - 1) - i];
      }
    }

    for (int i = 0; i < s.size(); i += 2 * (numRows - 1)) {
      if (i + numRows - 1 < s.size())
      answer += s[i + numRows - 1];
    }
    return answer;
  }
};
