// link to the problem: https://leetcode.com/problems/words-within-two-edits-of-dictionary/
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

// Runtime: 46 ms, faster than 61.48% of C++ online submissions for Words Within Two Edits of Dictionary.
// Memory Usage: 9.2 MB, less than 68.89% of C++ online submissions for Words Within Two Edits of Dictionary.

class Solution {
 public:
  std::vector<std::string> twoEditWords(std::vector<std::string>& queries, std::vector<std::string>& dictionary) {
    std::vector<std::string> answer;

    for (const auto& query : queries) {
      int minDistance = query.size();
      for (const auto& word : dictionary) {
        int distance = 0;
        for (int i = 0; i < word.size(); ++i)
          if (query[i] != word[i]) distance++;
        minDistance = std::min(minDistance, distance);
      }
      if (minDistance <= 2) answer.push_back(query);
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
