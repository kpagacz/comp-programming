// link to the problem: https://leetcode.com/problems/word-subsets/
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

// Runtime: 602 ms, faster than 23.37% of C++ online submissions for Word Subsets.
// Memory Usage: 162 MB, less than 18.20% of C++ online submissions for Word Subsets.

class Solution {
 public:
  std::vector<std::string> wordSubsets(std::vector<std::string>& words1, std::vector<std::string>& words2) {
    std::vector<std::string> answer;

    std::unordered_map<char, int> preprocessedWords;

    for (const auto& word : words2) {
      std::unordered_map<char, int> wordMap;
      for (const auto& c : word) wordMap[c]++;

      for (const auto& [c, occurences] : wordMap) preprocessedWords[c] = std::max(preprocessedWords[c], occurences);
    }

    for (const auto& word : words1) {
      std::unordered_map<char, int> wordMap;
      for (const auto& c : word) wordMap[c]++;
      bool universal = true;
      for (const auto& [c, occurences] : preprocessedWords)
        universal = universal && wordMap.find(c) != wordMap.end() && wordMap[c] >= occurences;
      if (universal) answer.push_back(word);
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
