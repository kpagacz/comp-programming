// link to the problem: https://leetcode.com/problems/number-of-matching-subsequences/
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

// Runtime: 280 ms, faster than 69.94% of C++ online submissions for Number of Matching Subsequences.
// Memory Usage: 34.5 MB, less than 96.14% of C++ online submissions for Number of Matching Subsequences.

class Solution {
 public:
  int numMatchingSubseq(std::string s, std::vector<std::string>& words) {
    int res = 0;
    for (const auto& word : words) {
      if (isSubsequence(s, word)) {
        ++res;
      }
    }
    return res;
  }

 private:
  std::unordered_map<std::string, bool> cache;
  bool isSubsequence(const std::string& s, const std::string& word) {
    if (cache.count(word)) {
      return cache[word];
    }
    auto sIt = s.begin();
    auto wordIt = word.begin();

    while (sIt != s.end() && wordIt != word.end()) {
      if (*sIt == *wordIt) {
        ++sIt;
        ++wordIt;
      } else {
        ++sIt;
      }
    }

    cache[word] = wordIt == word.end();
    return cache[word];
  }
};

int main(int argc, char** argv) {}
