// link to the problem: https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
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
  int findMaximumXOR(std::vector<int>& nums) {
    int result{0}, mask{0};
    std::unordered_set<int> prefixes;

    for (auto bits{30}; bits >= 0; bits--) {
      mask |= 1 << bits;
      for (const auto& n : nums) prefixes.insert(mask & n);

      int new_result = result;
      new_result |= 1 << bits;
      for (const auto& p : prefixes) {
        // Abusing the fact that
        // xor(a, b) = c -> xor(b,c) = a and xor(a,c) = b
        if (prefixes.find(new_result ^ p) != std::end(prefixes)) {
          result = new_result;
          break;
        }
      }
      prefixes.clear();
    }

    return result;
  }
};

int main(int argc, char** argv) {}
