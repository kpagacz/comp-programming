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
#include <unordered_map>
#include <unordered_set>
#include <vector>

// This passes faster than 50% of all c++ submissions

struct hash_pair {
    template <class T1, class T2>
    size_t operator()(const std::pair<T1, T2>& p) const
    {
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
  WordFilter(std::vector<std::string>& words) {
    prefixes.reserve(words.size() * 10);
    suffixes.reserve(words.size() * 10);
    for (auto i{0}; i < words.size(); i++) {
      std::string word = words[i];
      for (auto length{1}; length <= word.size(); length++) {
        std::string substring = word.substr(0, length);
        if (prefixes.find(substring) == prefixes.end()) prefixes[substring] = std::vector<int>();
        prefixes[substring].push_back(i);
      }

      std::reverse(word.begin(), word.end());
      for (auto length{1}; length <= word.size(); length++) {
        std::string substring = word.substr(0, length);
        if (suffixes.find(substring) == suffixes.end()) suffixes[substring] = std::vector<int>();
        suffixes[substring].push_back(i);
      }
    }
  }

  int f(std::string prefix, std::string suffix) {
    std::reverse(suffix.begin(), suffix.end());
    if (cache.find({prefix, suffix}) != cache.end()) return cache[{prefix, suffix}];

    if (prefixes.find(prefix) == prefixes.end() || suffixes.find(suffix) == suffixes.end()) return -1;
    std::sort(prefixes[prefix].begin(), prefixes[prefix].end());
    std::sort(suffixes[suffix].begin(), suffixes[suffix].end());

    std::vector<int> prefixes_words = prefixes[prefix];
    std::vector<int> suffixes_words = suffixes[suffix];

    std::vector<int> intersection;
    std::set_intersection(prefixes_words.begin(), prefixes_words.end(), suffixes_words.begin(), suffixes_words.end(),
                          std::back_inserter(intersection));
    if (intersection.empty()) {
      cache[{prefix, suffix}] = -1;
      return -1;
    } else {
      cache[{prefix, suffix}] = intersection.back();
      return intersection.back();
    }
  }

 private:
  std::unordered_map<std::string, std::vector<int>> prefixes, suffixes;
  std::unordered_map<std::pair<std::string, std::string>, int, hash_pair> cache;
};

int main(int argc, char** argv) {}
