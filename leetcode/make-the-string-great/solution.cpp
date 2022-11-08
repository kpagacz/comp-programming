// link to the problem:
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Make The String Great.
// Memory Usage: 6.1 MB, less than 97.69% of C++ online submissions for Make The String Great.

class Solution {
 public:
  std::string makeGood(std::string s) {
    bool changed = true;
    while (changed) {
      changed = false;
      for (int i = 1; i < s.size(); ++i)
        if ((std::islower(s[i]) && std::toupper(s[i]) == s[i - 1]) ||
            (s[i] == std::toupper(s[i - 1]) && std::islower(s[i - 1]))) {
          s[i] = s[i - 1] = '$';
          changed = true;
        }
      s.erase(std::remove(s.begin(), s.end(), '$'), s.end());
    }
    return s;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::string> cases;
  cases.push_back("leEeetcode");
  cases.push_back("abBAcC");
  cases.push_back("kkdsFuqUfSDKK");
  for (auto word : cases) std::cout << s.makeGood(word) << '\n';
}
