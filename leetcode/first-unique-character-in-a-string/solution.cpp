// link to the problem: https://leetcode.com/problems/first-unique-character-in-a-string/
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

// Runtime: 23 ms, faster than 93.65% of C++ online submissions for First Unique Character in a String.
// Memory Usage: 10.5 MB, less than 98.28% of C++ online submissions for First Unique Character in a String.

constexpr int NO_OCCURENCE = -1;
constexpr int NOT_UNIQUE = -2;

class Solution {
 public:
  int firstUniqChar(std::string s) {
    std::vector<int> letterPositions(26, NO_OCCURENCE);
    for (int i = 0; i < s.size(); ++i) {
      auto& position = letterPositions[s[i] - 'a'];
      if (position == NOT_UNIQUE)
        continue;
      else if (position == NO_OCCURENCE)
        position = i;
      else
        position = NOT_UNIQUE;
    }

    std::sort(letterPositions.begin(), letterPositions.end());
    const auto& unique = std::lower_bound(letterPositions.begin(), letterPositions.end(), 0);
    if (unique == letterPositions.end())
      return -1;
    else
      return *unique;
  };
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::string> cases{"leetcode", "loveleetcode", "aabb", "ll"};

  for (auto& test : cases) std::cout << s.firstUniqChar(test) << '\n';
}
