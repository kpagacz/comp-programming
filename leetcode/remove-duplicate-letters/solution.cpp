// link to the problem: https://leetcode.com/problems/remove-duplicate-letters/
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
  std::string removeDuplicateLetters(std::string s) {
    std::vector<int> left(26, 0);
    for (const auto& c : s) left[c - 'a']++;
    std::stack<char> letters;
    std::vector<bool> in_letters(26, false);
    for (const auto& c : s) {
      left[c - 'a']--;
      if (in_letters[c - 'a']) continue;
      while (!letters.empty() && left[letters.top() - 'a'] && letters.top() > c) {
        in_letters[letters.top() - 'a'] = false;
        letters.pop();
      }
        letters.push(c);
        in_letters[c - 'a'] = true;
    }

    std::string answer;
    while (!letters.empty()) {
      answer += letters.top();
      letters.pop();
    }
    std::reverse(answer.begin(), answer.end());
    return answer;
  }
};

int main(int argc, char** argv) {}
