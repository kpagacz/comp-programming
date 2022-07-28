// link to the problem: https://leetcode.com/problems/valid-anagram/
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
  bool isAnagram(std::string s, std::string t) {
    std::unordered_map<char, int> firstWord, secondWord;
    for (const auto& c : s) firstWord[c]++;
    for (const auto& c : t) secondWord[c]++;
    return firstWord == secondWord;
  }
};
int main(int argc, char** argv) {}
