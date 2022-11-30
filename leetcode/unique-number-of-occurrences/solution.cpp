// link to the problem: https://leetcode.com/problems/unique-number-of-occurrences/
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

// Runtime: 4 ms, faster than 69.41% of C++ online submissions for Unique Number of Occurrences.
// Memory Usage: 8.7 MB, less than 6.37% of C++ online submissions for Unique Number of Occurrences.

class Solution {
 public:
  bool uniqueOccurrences(std::vector<int>& arr) {
    std::vector<int> occurrences(2001, 0);
    for (const auto& num : arr) occurrences[num + 1000]++;

    std::vector<bool> numberOfOccurrences(2001, false);
    for (const auto& occ : occurrences)
      if (occ != 0 && numberOfOccurrences[occ]) return false;
      else numberOfOccurrences[occ] = true;
    return true;
  }
};

int main(int argc, char** argv) {}
