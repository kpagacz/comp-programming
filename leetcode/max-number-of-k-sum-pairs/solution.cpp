// link to the problem: https://leetcode.com/problems/max-number-of-k-sum-pairs/
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
  std::unordered_map<int, int> occurences;
  int maxOperations(std::vector<int>& nums, int k) {
    for (const auto& n : nums) {
      if (occurences.count(n) > 0)
        occurences[n]++;
      else
        occurences[n] = 1;
    }

    int pairs = 0;
    auto counter = [&](const auto& pair) {
      if (occurences.count(k - pair->first) > 0) pairs += std::min(occurences[k - pair->first], pair->second);
    };
    std::for_each(occurences.begin(), occurences.end(), counter);

    return pairs / 2;
  }
};

int main(int argc, char** argv) {}
