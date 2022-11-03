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

// Runtime: 820 ms, faster than 49.69% of C++ online submissions for Longest Palindrome by Concatenating Two Letter
// Words. Memory Usage: 167.9 MB, less than 81.81% of C++ online submissions for Longest Palindrome by Concatenating Two
// Letter Words.

class Solution {
 public:
  int longestPalindrome(std::vector<std::string>& words) {
    std::unordered_map<std::string, int> wordsMap;

    for (const auto& word : words) wordsMap[word]++;

    int pairs = 0;
    bool hasLonelyPalindrome = false;
    for (const auto& [word, count] : wordsMap) {
      if (count == 0) continue;
      auto reversed{word};
      std::reverse(reversed.begin(), reversed.end());

      while (word != reversed && wordsMap[word] > 0 && wordsMap.count(reversed) > 0 && wordsMap[reversed] > 0) {
        wordsMap[reversed]--;
        wordsMap[word]--;
        pairs++;
      }

      while (word == reversed && wordsMap[word] >= 2) {
        pairs++;
        wordsMap[word] -= 2;
      }

      if (word == reversed && wordsMap[word] == 1) hasLonelyPalindrome = true;
    }

    return hasLonelyPalindrome ? (pairs << 2) + 2 : pairs << 2;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<std::string>> cases;
  cases.push_back({"lc", "cl", "gg"});
  cases.push_back({"ab", "ty", "yt", "lc", "cl", "ab"});
  cases.push_back({"cc", "ll", "xx"});
  cases.push_back({"cl"});

  for (auto words : cases) std::cout << s.longestPalindrome(words) << '\n';
}
