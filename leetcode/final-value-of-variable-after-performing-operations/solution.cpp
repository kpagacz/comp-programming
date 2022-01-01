// link to the problem: https://leetcode.com/problems/final-value-of-variable-after-performing-operations
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
 public:
  int finalValueAfterOperations(std::vector<std::string>& operations) {
    std::unordered_map<std::string, int> increments{{"X++", 0}, {"++X", 0}, {"--X", 0}, {"X--", 0}};
    std::for_each(operations.begin(), operations.end(), [&](const auto& s) { increments[s]++; });
    return increments["X++"] + increments["++X"] - increments["--X"] - increments["X--"];
  }
};

int main(int argc, char** argv) {}
