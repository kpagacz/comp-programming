// link to the problem: https://leetcode.com/problems/stamping-the-sequence/
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

// Runtime: 11 ms, faster than 87.70% of C++ online submissions for Stamping The Sequence.
// Memory Usage: 7.5 MB, less than 76.23% of C++ online submissions for Stamping The Sequence.

class Solution {
 public:
  std::vector<int> movesToStamp(std::string stamp, std::string target) {
    std::vector<int> answer;

    const std::string REPLACEMENT(stamp.size(), '?');

    bool matchedOnce = true;
    while (matchedOnce) {
      matchedOnce = false;
      for (int i = 0; i <= target.size() - stamp.size(); ++i) {
        if (partialMatch(stamp, target, i)) {
          matchedOnce = true;
          target.replace(i, REPLACEMENT.size(), REPLACEMENT);
          answer.push_back(i);
        }
      }
    }

    if (target.find_first_not_of("?") != std::string::npos) return {};

    std::reverse(answer.begin(), answer.end());
    return answer;
  }

  bool partialMatch(const std::string& stamp, std::string& target, const int& index) {
    bool matched = false;
    for (int i = 0; i < stamp.size(); ++i) {
      if (target[index + i] == '?') continue;
      if (target[index + i] != stamp[i]) return false;
      matched = true;
    }
    return matched;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<std::string, std::string>> tests{
      {"abc", "ababc"}, {"abca", "aabcaca"}, {"abc", "aaaaaaaaabcc"}};
  for (auto [stamp, target] : tests) {
    const auto& res = s.movesToStamp(stamp, target);
    std::copy(res.begin(), res.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << "\n";
  }
}
