// link to the problem: https://leetcode.com/problems/first-letter-to-appear-twice/
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

class Solution {
 public:
  char repeatedCharacter(std::string s) {
    std::vector<bool> letters(26, false);
    for (const auto& c : s) {
      if (letters[c - 'a'])
        return c;
      else
        letters[c - 'a'] = true;
    }
    return 'a';
  }
};

int main(int argc, char** argv) {}
