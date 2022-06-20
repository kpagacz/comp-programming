// link to the problem: https://leetcode.com/problems/short-encoding-of-words/
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
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <vector>

class Solution {
 public:
  int minimumLengthEncoding(std::vector<std::string>& words) {
    std::string answer;
    std::sort(words.begin(), words.end(),
              [](const auto& first, const auto& second) { return first.size() > second.size(); });
    std::unordered_set<std::string_view> suffixes;
    for (int word_index{0}; word_index < words.size(); word_index++) {
      const auto& word = words[word_index];
      if (suffixes.find(word) == suffixes.end()) {
        answer += word + "#";
        auto first = word.data();
        for (int i{0}; i < word.size(); i++) {
          std::string_view view(first + i, word.size() - i);
          suffixes.insert(view);
        }
      }
    }

    return answer.size();
  }
};

int main(int argc, char** argv) {}
