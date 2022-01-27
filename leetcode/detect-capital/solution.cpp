// link to the problem: https://leetcode.com/problems/detect-capital/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>
#include <cctype>

class Solution {
public:
    bool detectCapitalUse(std::string word) {
      int capital_letters = std::count_if(word.begin(), word.end(), [](const auto &c)
                                          { return std::isupper(c); });
      return word.size() == capital_letters || capital_letters == 0 || capital_letters == 1 && std::isupper(word[0]);
    }
};

int main(int argc, char** argv) {}
