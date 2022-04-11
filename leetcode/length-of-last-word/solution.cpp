// link to the problem: https://leetcode.com/problems/length-of-last-word/
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
  int lengthOfLastWord(std::string s) {
    auto word_end = s.find_first_not_of(" ");
    auto word_start = s.find_last_of(" ", word_end);
    if (word_end == std::string::npos) return 0;
    if (word_start == std::string::npos) return s.size();
    return word_end - word_start;
  }
};


// another solution
class Solution {
 public:
  int lengthOfLastWord(std::string s) {
    int word_end = s.size() - 1;
    while (s[word_end] == ' ') {
      word_end--;
    }
    // there is at least one word, so no need to check if word_end == -1
    int word_start = word_end;
    while(word_start >= 0 && s[word_start] != ' ') {
      word_start--;
    }

    return word_end - word_start;
  }
};

int main(int argc, char** argv) {}
