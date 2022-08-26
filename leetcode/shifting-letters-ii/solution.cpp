// link to the problem: https://leetcode.com/problems/shifting-letters-ii/
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

// Runtime: 1331 ms, faster than 20.00% of C++ online submissions for Shifting Letters II.
// Memory Usage: 93.3 MB, less than 60.00% of C++ online submissions for Shifting Letters II.

class Solution {
 public:
  std::string shiftingLetters(std::string s, std::vector<std::vector<int>>& shifts) {
    std::vector<int> shiftsMagnitude(s.size() + 1, 0);
    for (const auto& shift : shifts) {
      if (shift[2]) {
        shiftsMagnitude[shift[0]]++;
        shiftsMagnitude[shift[1] + 1]--;
      } else {
        shiftsMagnitude[shift[0]]--;
        shiftsMagnitude[shift[1] + 1]++;
      }
    }

    std::partial_sum(shiftsMagnitude.begin(), shiftsMagnitude.end(), shiftsMagnitude.begin());
    for (int i = 0; i < s.size(); ++i) {
      while (shiftsMagnitude[i] < 0) shiftsMagnitude[i] += 26;
      s[i] = ((s[i] - 'a' + shiftsMagnitude[i]) % 26) + 'a';
    }
    return s;
  }
};

int main(int argc, char** argv) {}
