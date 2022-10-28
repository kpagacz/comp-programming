// link to the problem: https://leetcode.com/problems/group-anagrams
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

// Runtime: 71 ms, faster than 58.95% of C++ online submissions for Group Anagrams.
// Memory Usage: 19.4 MB, less than 87.73% of C++ online submissions for Group Anagrams.

struct AnagramHash {
  std::vector<int> primes{2,  3,  5,  7,  11, 13, 17, 19, 23, 29, 31, 37,  41, 43,
                          47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103};
  const int MODULO = (2 << 29) - 1;  // Mersenne prime
  std::size_t operator()(const std::basic_string<char>& word) const {
    std::size_t hash = 1;
    for (const char& c : word) {
      hash *= primes[c - 'a'];
      hash = hash % MODULO;
    }
    return hash;
  }
};

struct AnagramEqual {
  bool operator()(const std::basic_string<char>& first, const std::basic_string<char>& second) const {
    auto firstCopy(first), secondCopy(second);
    std::sort(firstCopy.begin(), firstCopy.end());
    std::sort(secondCopy.begin(), secondCopy.end());
    return firstCopy == secondCopy;
  }
};

class Solution {
 public:
  std::vector<std::vector<std::string>> groupAnagrams(std::vector<std::string>& strs) {
    std::unordered_map<std::string, std::vector<std::string>, AnagramHash, AnagramEqual> anagrams;
    for (const auto& str : strs) anagrams[str].push_back(str);

    std::vector<std::vector<std::string>> answer;
    for (const auto& [key, words] : anagrams) answer.push_back(words);
    return answer;
  }
};

int main(int argc, char** argv) { Solution s; }
