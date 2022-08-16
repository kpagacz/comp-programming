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

// Runtime: 82 ms, faster than 26.81% of C++ online submissions for First Unique Character in a String.
// Memory Usage: 10.8 MB, less than 44.39% of C++ online submissions for First Unique Character in a String.

class Solution {
 public:
  int firstUniqChar(std::string s) {
    std::unordered_map<char, int> letterPositions;
    for (int i = 0; i < s.size(); ++i)
      if (letterPositions.count(s[i]) == 0)
        letterPositions[s[i]] = i;
      else
        letterPositions[s[i]] = -1;
    int min = INT32_MAX;
    for (const auto& [c, position] : letterPositions)
      if (position != -1 && position < min) min = position;
    if (min == INT32_MAX)
      return -1;
    else
      return min;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::string> cases{"leetcode", "loveleetcode", "aabb", "ll"};

  for (auto& test : cases) std::cout << s.firstUniqChar(test) << '\n';
}
