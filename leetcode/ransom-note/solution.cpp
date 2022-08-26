// link to the problem: https://leetcode.com/problems/ransom-note/
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

// Runtime: 19 ms, faster than 72.14% of C++ online submissions for Ransom Note.
// Memory Usage: 8.7 MB, less than 72.69% of C++ online submissions for Ransom Note.

class Solution {
 public:
  bool canConstruct(std::string ransomNote, std::string magazine) {
    std::vector<int> letters(26, 0);
    for (const auto& letter : magazine) letters[letter - 'a']++;
    for (const auto& letter : ransomNote) letters[letter - 'a']--;
    return std::all_of(letters.begin(), letters.end(), [](const auto& count) { return count >= 0; });
  }
};

int main(int argc, char** argv) {}
