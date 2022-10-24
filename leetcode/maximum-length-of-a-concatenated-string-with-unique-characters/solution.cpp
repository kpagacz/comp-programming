// link to the problem: https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
#include <algorithm>
#include <array>
#include <bitset>
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

// Runtime: 11 ms, faster than 86.03% of C++ online submissions for Maximum Length of a Concatenated String with Unique
// Characters. Memory Usage: 8.2 MB, less than 86.03% of C++ online submissions for Maximum Length of a Concatenated
// String with Unique Characters.

class Solution {
 public:
  int maxLength(std::vector<std::string>& arr) {
    std::vector<std::bitset<32>> strings;
    strings.reserve(arr.size());
    for (const auto& word : arr) {
      std::bitset<32> bits(0);
      bool repeat = false;
      for (const char& c : word) {
        repeat = repeat || bits[c - 'a'];
        bits.set(c - 'a');
      }
      if (repeat) continue;
      strings.push_back(bits);
    }

    std::size_t max = 0;
    maxLength(strings, 0, std::bitset<32>(0), max);
    return max;
  }

  void maxLength(const std::vector<std::bitset<32>>& strings, const int& index,
                 const std::bitset<32>& currentConcatenation, std::size_t& max) {
    max = std::max(max, currentConcatenation.count());
    if (index >= strings.size()) return;
    if ((strings[index] & currentConcatenation).none())
      maxLength(strings, index + 1, strings[index] | currentConcatenation, max);

    maxLength(strings, index + 1, currentConcatenation, max);
  }
};

int main(int argc, char** argv) {}
