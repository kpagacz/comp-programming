// link to the problem: https://leetcode.com/problems/odd-string-difference/
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

// Runtime: 4 ms, faster than 75.30% of C++ online submissions for Odd String Difference.
// Memory Usage: 7.3 MB, less than 86.22% of C++ online submissions for Odd String Difference.

class Solution {
 public:
  std::string oddString(std::vector<std::string>& words) {
    const auto& base = words[0][0];
    std::vector<int> offsets;
    offsets.reserve(words.size());
    for (auto& word : words) {
      const int& offset = word[0] - base;
      offsets.push_back(offset);
      for (auto& c : word) c = ((26 + (c - 'a') - offset) % 26) + 'a';
    }

    int different;
    if (words[0] == words[1]) {
      for (int i = 1; i < words.size(); ++i)
        if (words[i] != words[i - 1]) {
          different = i;
          break;
        }
    } else {
      if (words[0] == words[2])
        different = 1;
      else
        different = 0;
    }

    std::string differentWord{words[different]};
    for (auto& c : differentWord) c = ((26 + (c - 'a') + offsets[different]) % 26) + 'a';
    return differentWord;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<std::string>> cases;

  cases.push_back({"adc", "wzy", "abc"});
  cases.push_back({"aaa", "bob", "ccc", "ddd"});
  for (auto words : cases) std::cout << s.oddString(words) << '\n';
}
