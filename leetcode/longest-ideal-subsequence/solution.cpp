// link to the problem: https://leetcode.com/problems/longest-ideal-subsequence/
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

// Runtime: 149 ms, faster than 85.71% of C++ online submissions for Longest Ideal Subsequence.
// Memory Usage: 10.3 MB, less than 85.71% of C++ online submissions for Longest Ideal Subsequence.

class Solution {
 public:
  int longestIdealString(std::string s, int k) {
    std::vector<int> longestIdeal(26, 0);
    int answer = 0;
    for (int i = 0; i < s.size(); ++i) {
      auto maxLength = 0;
      for (int j = s[i] - k - 'a'; j <= s[i] + k - 'a'; ++j) {
        if (j >= 0 && j < longestIdeal.size()) maxLength = std::max(maxLength, longestIdeal[j]);
      }
      longestIdeal[s[i] - 'a'] = maxLength + 1;
      // std::copy(longestIdeal.begin(), longestIdeal.end(), std::ostream_iterator<int>(std::cout, " "));
      // std::cout << '\n';
      answer = std::max(answer, longestIdeal[s[i] - 'a']);
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  std::vector<std::pair<std::string, int>> cases{{"acfgbd", 2}, {"cafbgczdea", 2}, {"pvjcci", 4}};
  Solution s;
  for (auto& [test, diff] : cases) {
    std::cout << "CASE: " << test << " diff: " << diff << '\n';
    std::cout << s.longestIdealString(test, diff) << '\n';
  }
}
