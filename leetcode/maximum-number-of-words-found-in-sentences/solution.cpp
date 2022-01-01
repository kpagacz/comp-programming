// link to the problem: https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <string>
#include <vector>

class Solution {
 public:
  int mostWordsFound(std::vector<std::string>& sentences) {
    int max{0};
    std::for_each(sentences.begin(), sentences.end(), [&](const auto& s) {
      auto words = std::count(s.begin(), s.end(), ' ') + 1;
      if (words > max) max = words;
    });
    return max;
  }
};

int main(int argc, char** argv) {}
