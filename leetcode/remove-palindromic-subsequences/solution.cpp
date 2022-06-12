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

class Solution {
 public:
  int removePalindromeSub(std::string s) {
    if (isPalindrome(s))
      return 1;
    else
      return 2;
  }

 private:
  bool isPalindrome(const std::string& s) {
    bool answer = true;
    for (int i = 0; i < s.length() / 2; i++) {
      if (s[i] != s[s.length() - i - 1]) {
        answer = false;
        break;
      }
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
