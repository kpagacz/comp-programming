// link to the problem: https://leetcode.com/problems/prefix-and-suffix-search/
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

// This is a similar idea as solution.cpp but uses string_views instead

struct hash_pair {
  template <class T1, class T2>
  size_t operator()(const std::pair<T1, T2>& p) const {
    auto hash1 = std::hash<T1>{}(p.first);
    auto hash2 = std::hash<T2>{}(p.second);

    if (hash1 != hash2) {
      return hash1 ^ hash2;
    }

    // If hash1 == hash2, their XOR is zero.
    return hash1;
  }
};

class WordFilter {
 public:
  WordFilter(std::vector<std::string>& words) : dictionary(std::move(words)) {
    for (int word_index = dictionary.size() - 1; word_index >= 0; --word_index) {
      const auto& word = dictionary[word_index];
      const auto begin = word.data();
      const auto size = word.size();
      for (int i{1}; i <= word.size(); ++i) {
        const std::string_view prefix(begin, i);
        for (int j{1}; j <= word.size(); ++j) {
          const std::string_view suffix(begin + size - j, j);
          answers.try_emplace({prefix, suffix}, word_index);
        }
      }
    }
  }

  int f(std::string prefix, std::string suffix) {
    if (answers.count({prefix, suffix}) == 1) return answers[{prefix, suffix}];
    return -1;
  }

 private:
  std::vector<std::string> dictionary;
  std::unordered_map<std::pair<std::string_view, std::string_view>, int, hash_pair> answers;
};

int main(int argc, char** argv) {}
