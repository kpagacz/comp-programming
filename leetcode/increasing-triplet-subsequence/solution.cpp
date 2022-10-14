// link to the problem: https://leetcode.com/problems/increasing-triplet-subsequence/
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

// Runtime: 90 ms, faster than 77.54% of C++ online submissions for Increasing Triplet Subsequence.
// Memory Usage: 61.6 MB, less than 24.04% of C++ online submissions for Increasing Triplet Subsequence.

class Solution {
 public:
  bool increasingTriplet(std::vector<int>& nums) {
    std::vector<int> array;
    array.reserve(2);

    int memory = INT32_MAX;

    for (const auto& num : nums) {
      if (array.empty()) {
        array.push_back(num);
        continue;
      }

      if (array.size() == 1) {
        if (num > memory) return true;
        if (num <= array.back())
          array.back() = num;
        else
          array.push_back(num);
      }

      if (array.size() == 2) {
        if (num > array.back()) return true;
        if (num < array[0]) {
          memory = array.back();
          array.pop_back();
          array[0] = num;
          continue;
        }
        if (num < array.back()) array.back() = num;
      }
    }

    return false;
  }
};

int main(int argc, char** argv) {}
