// link to the problem: https://leetcode.com/problems/construct-smallest-number-from-di-string/
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

// Runtime: 5 ms, faster than 43.14% of C++ online submissions for Construct Smallest Number From DI String.
// Memory Usage: 6 MB, less than 68.69% of C++ online submissions for Construct Smallest Number From DI String.

class Solution {
 public:
  std::string smallestNumber(std::string pattern) {
    pattern += pattern.back();
    std::string answer{pattern};
    char digit = '0';
    int backlog = 0;
    for (int i = 0; i < pattern.size(); ++i)
      if (pattern[i] == 'I') {
        answer[i] = ++digit;
        for (int j = 1; j <= backlog; ++j) answer[i - j] = digit + j;
        digit += backlog;
        backlog = 0;
      } else
        backlog++;
    for (int j = 1; j <= backlog; ++j) answer[answer.size() - j] = digit + j;
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::string> tests{"DIDI", "DDD", "III", "I", "D", "IIIDIDDD", "DID"};
  for (auto& test : tests) {
    std::cout << s.smallestNumber(test) << '\n';
  }
}
