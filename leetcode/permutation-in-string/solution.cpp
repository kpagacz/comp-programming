// link to the problem: https://leetcode.com/problems/permutation-in-string/
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
  bool checkInclusion(std::string s1, std::string s2) {
    std::vector<int> pattern(26, 0);
    std::vector<int> encountered(26, 0);
    if (s1.size() > s2.size()) return false;
    for (const auto& c : s1) pattern[c - 'a']++;
    for (auto i{0}; i < s1.size(); i++) {
      encountered[s2[i] - 'a']++;
    }
    if (pattern == encountered) return true;
    int left = 0, right = s1.size() - 1;
    bool matched = false;
    while (right + 1 < s2.size()) {
      encountered[s2[left++] - 'a']--;
      encountered[s2[++right] - 'a']++;
      matched |= encountered == pattern;
    }
    return matched;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << std::boolalpha << s.checkInclusion("ab", "eidbaoo");
}
