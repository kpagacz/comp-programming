// link to the problem: https://leetcode.com/problems/delete-operation-for-two-strings/
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

class Solution {
 public:
  int minDistance(std::string word1, std::string word2) {
    return word1.size() + word2.size() - 2 * longestCommonSubsequence(word1, word2);
  }

  int longestCommonSubsequence(const std::string& text1, const std::string& text2) {
    int lcs[text1.size() + 1][text2.size() + 1];

    for (int i{0}; i < text1.size() + 1; i++) {
      for (int j{0}; j < text2.size() + 1; j++) {
        if (i == 0 || j == 0) {
          lcs[i][j] = 0;
          continue;
        }
        if (text1[i - 1] == text2[j - 1]) {
          lcs[i][j] = lcs[i - 1][j - 1] + 1;
        } else {
          lcs[i][j] = std::max(lcs[i][j - 1], lcs[i - 1][j]);
        }
      }
    }
    return lcs[text1.size()][text2.size()];
  }
};

int main(int argc, char** argv) {}
