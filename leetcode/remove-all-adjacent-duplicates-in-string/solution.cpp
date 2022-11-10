// link to the problem: https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
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

// Runtime: 31 ms, faster than 79.18% of C++ online submissions for Remove All Adjacent Duplicates In String.
// Memory Usage: 10.4 MB, less than 84.64% of C++ online submissions for Remove All Adjacent Duplicates In String.

class Solution {
 public:
  std::string removeDuplicates(std::string s) {
    std::stack<char> letters;
    for (const auto& c : s) {
      if (!letters.empty() && c == letters.top()) letters.pop();
      else letters.push(c);
    }
    s = "";
    while (!letters.empty()) {
      s += letters.top();
      letters.pop();
    }
    std::reverse(s.begin(), s.end());
    return s;
  }
};

int main(int argc, char** argv) {
  Solution sol;
  std::vector<std::string> cases;
  cases.push_back("abbac");
  cases.push_back("aa");
  cases.push_back("abc");
  for (auto s : cases) std::cout << sol.removeDuplicates(s) << '\n';
}
