// link to the problem: https://leetcode.com/problems/is-subsequence/
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
  bool isSubsequence(std::string s, std::string t) {
    int i = 0, j = 0;
    while (j < t.size()) {
      if (i == s.size()) return true;
      else if (s[i] == t[j])
        i++;
      j++;
    }
    return i == s.size();
  }
};

int main(int argc, char** argv) {}
