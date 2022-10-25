// link to the problem: https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
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

// Runtime: 11 ms, faster than 38.92% of C++ online submissions for Check If Two String Arrays are Equivalent.
// Memory Usage: 12.4 MB, less than 16.80% of C++ online submissions for Check If Two String Arrays are Equivalent.

class Solution {
 public:
  bool arrayStringsAreEqual(std::vector<std::string>& word1, std::vector<std::string>& word2) {
    return std::accumulate(word1.begin(), word1.end(), std::string()) ==
           std::accumulate(word2.begin(), word2.end(), std::string());
  }
};

int main(int argc, char** argv) {}
