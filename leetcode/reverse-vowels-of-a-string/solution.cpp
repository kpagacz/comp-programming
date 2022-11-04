// link to the problem: https://leetcode.com/problems/reverse-vowels-of-a-string/
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
  // Runtime: 15 ms, faster than 55.02% of C++ online submissions for Reverse Vowels of a String.
  // Memory Usage: 8.7 MB, less than 6.35% of C++ online submissions for Reverse Vowels of a String.
  // std::string reverseVowels(std::string s) {
  //   std::stack<char> vowelsStack;
  //   for (const auto& c : s)
  //     if (vowels.count(c)) vowelsStack.push(c);

  //   for (auto& c : s)
  //     if (vowels.count(c)) {
  //       c = vowelsStack.top();
  //       vowelsStack.pop();
  //     }

  //   return s;
  // }

  // Runtime: 13 ms, faster than 62.90% of C++ online submissions for Reverse Vowels of a String.
  // Memory Usage: 7.9 MB, less than 37.49% of C++ online submissions for Reverse Vowels of a String.
  std::string reverseVowels(std::string s) {
    int left = 0, right = s.size() - 1;
    while (left < right) {
      if (vowels.count(s[left]) == 0)
        left++;
      else if (vowels.count(s[right]) == 0)
        right--;
      else {
        std::swap(s[left++], s[right--]);
      }
    }

    return s;
  }
  const std::unordered_set<char> vowels{'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'};
};

int main(int argc, char** argv) {}
