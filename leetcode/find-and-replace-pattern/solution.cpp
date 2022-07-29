// link to the problem: https://leetcode.com/problems/find-and-replace-pattern/
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

// This was an interesting excercise in hashing etc, but has aweful constant factor of complexity.
// I have a simpler solution in mind.

// Runtime: 16 ms, faster than 7.40% of C++ online submissions for Find and Replace Pattern.
// Memory Usage: 12.8 MB, less than 5.07% of C++ online submissions for Find and Replace Pattern.

template <class T>
inline void hashCombine(std::size_t& seed, const T& v) {
  std::hash<T> hasher;
  seed ^= hasher(v) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
}

template <class T>
struct RangeHash {
  inline std::size_t operator()(const T& c) const {
    std::size_t seed = 0;
    for (typename T::const_iterator it = c.begin(), end = c.end(); it != end; ++it) {
      hashCombine<typename T::value_type>(seed, *it);
    }
    return seed;
  }
};

class Solution {
 public:
  std::vector<std::string> findAndReplacePattern(std::vector<std::string>& words, std::string pattern) {
    std::vector<std::string> answer;
    std::unordered_map<char, std::unordered_set<int>> patternPositions;
    for (int i = 0; i < pattern.size(); ++i) patternPositions[pattern[i]].insert(i);
    std::unordered_set<std::unordered_set<int>, RangeHash<std::unordered_set<int>>> patternPositionsSet;
    for (const auto& [key, set] : patternPositions) patternPositionsSet.insert(set);

    for (const auto& word : words) {
      std::unordered_map<char, std::unordered_set<int>> wordPositions;
      for (int i = 0; i < word.size(); ++i) wordPositions[word[i]].insert(i);
      std::unordered_set<std::unordered_set<int>, RangeHash<std::unordered_set<int>>> wordPositionsSet;
      for (const auto& [key, set] : wordPositions) wordPositionsSet.insert(set);
      if (patternPositionsSet == wordPositionsSet) answer.push_back(word);
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
