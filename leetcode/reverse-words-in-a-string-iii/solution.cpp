// link to the problem: https://leetcode.com/problems/reverse-words-in-a-string-iii/

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

// Runtime: 44 ms, faster than 18.98% of C++ online submissions for Reverse Words in a String III.
// Memory Usage: 13.9 MB, less than 15.77% of C++ online submissions for Reverse Words in a String III.

class Solution {
 public:
  std::string reverseWords(std::string s) {
    std::string answer;
    std::stringstream stream(s);
    std::string word;
    while (std::getline(stream, word, ' ')) {
      std::reverse(word.begin(), word.end());
      answer += word + " ";
    }
    answer.erase(answer.size() - 1);
    return answer;
  }
};

int main(int argc, char** argv) {}
