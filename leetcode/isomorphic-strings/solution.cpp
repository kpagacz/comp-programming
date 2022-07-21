// link to the problem: https://leetcode.com/problems/isomorphic-strings/
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

// Runtime: 10 ms, faster than 56.98% of C++ online submissions for Isomorphic Strings.
// Memory Usage: 10.3 MB, less than 5.03% of C++ online submissions for Isomorphic Strings.

struct VectorHashFunction {
  template <typename T>
  std::size_t operator()(const std::vector<T>& v) const {
    std::hash<T> hashFunction;
    std::size_t answer = 0;
    for (const auto& element : v) answer ^= hashFunction(element) + 0x9e3779b9 + (answer << 6) + (answer >> 2);
    return answer;
  }
};

class Solution {
 public:
  bool isIsomorphic(std::string s, std::string t) {
    if (s.size() != t.size()) return false;
    std::unordered_map<char, std::vector<int>> firstStringPositions, secondStringPositions;
    for (int i = 0; i < s.size(); ++i) {
      firstStringPositions[s[i]].push_back(i);
      secondStringPositions[t[i]].push_back(i);
    }

    std::unordered_set<std::vector<int>, VectorHashFunction> firstStringPositionsSet, secondStringPositionsSet;
    for (auto& firstStringPositionsPair : firstStringPositions) {
      firstStringPositionsSet.insert(firstStringPositionsPair.second);
    }
    for (auto& secondStringPositionsPair : secondStringPositions) {
      secondStringPositionsSet.insert(secondStringPositionsPair.second);
    }
    return firstStringPositionsSet == secondStringPositionsSet;
  }
};

int main(int argc, char** argv) {}
