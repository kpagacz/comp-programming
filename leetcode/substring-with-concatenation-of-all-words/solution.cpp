// link to the problem: https://leetcode.com/problems/substring-with-concatenation-of-all-words/
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

// this passes and is faster than 34% and uses less memory than 98% of all solutions

class Solution {
 public:
  std::vector<int> findSubstring(std::string s, std::vector<std::string>& words) {
    const int totalLength = words.size() * words[0].size();
    const int wordSize = words[0].size();

    std::unordered_map<std::string_view, int> wordsTracker;
    wordsTracker.reserve(words.size());
    for (const auto& word : words)
      if (wordsTracker.find(word) == wordsTracker.end())
        wordsTracker[word] = 1;
      else
        wordsTracker[word]++;

    const auto wordsTrackerOriginal = wordsTracker;


    std::vector<int> answer;
    auto wordBegin = s.data();
    for (int i{0}; i + totalLength <= s.size(); i++) {
      bool foundSubstring = true;
      for (int j{0}; j < words.size(); j++) {
        std::string_view word(wordBegin + i + wordSize * j, wordSize);
        if (wordsTracker.find(word) == wordsTracker.end() || wordsTracker[word] == 0) {
          foundSubstring = false;
          break;
        }
        wordsTracker[word]--;
      }
      if (foundSubstring) answer.push_back(i);
      for (auto& [key, value] : wordsTracker) value = wordsTrackerOriginal.at(key);
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
