// link to the problem: https://leetcode.com/problems/time-needed-to-rearrange-a-binary-string/
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

// Runtime: 141 ms, faster than 30.00% of C++ online submissions for Time Needed to Rearrange a Binary String.
// Memory Usage: 6.7 MB, less than 10.00% of C++ online submissions for Time Needed to Rearrange a Binary String.

class Solution {
 public:
  int secondsToRemoveOccurrences(std::string s) {
    int seconds = 0;
    bool replaced = true;

    while (replaced) {
      replaced = false;
      int index = 1;
      while (index < s.size()) {
        if (s[index - 1] == '0' && s[index] == '1') {
          s[index - 1] = '1';
          s[index] = '0';
          replaced = true;
          index += 2;
        } else
          index++;
      }
      if (replaced) seconds++;
      index = 1;
    }

    return seconds;
  }
};

int main(int argc, char** argv) {}
