// link to the problem: https://leetcode.com/problems/find-the-difference/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
public:
    char findTheDifference(std::string s, std::string t) {
      std::vector<int> pattern(26, 0);
      for(const auto& c : s)
        pattern[c - 'a']++;

      for(const auto& c : t) if(--pattern[c - 'a'] < 0)
          return c;
      return 0;
    }
};

int main(int argc, char** argv) {}
