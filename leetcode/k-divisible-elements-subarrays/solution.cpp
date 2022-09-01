// link to the problem: https://leetcode.com/problems/k-divisible-elements-subarrays/
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

// Runtime: 1005 ms, faster than 64.43% of C++ online submissions for K Divisible Elements Subarrays.
// Memory Usage: 406.6 MB, less than 5.31% of C++ online submissions for K Divisible Elements Subarrays.

template <typename T>
struct VectorHash {
  std::size_t operator()(std::vector<T> arr) const {
    std::hash<T> hasher;
    std::size_t seed = 0;
    for (const auto& el : arr) seed ^= hasher(el) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
    return seed;
  }
};

class Solution {
 public:
  int countDistinct(std::vector<int>& nums, int k, int p) {
    std::vector<int> numCopy = nums;
    for (auto& num : nums) num = num % p == 0;
    std::vector<int> prefixSums(nums.size() + 1);
    std::partial_sum(nums.begin(), nums.end(), prefixSums.begin() + 1);
    std::unordered_set<std::vector<int>, VectorHash<int>> subarrays;
    for (int i = 0; i < prefixSums.size(); ++i) {
      std::vector<int> subarray;
      for (int j = i + 1; j < prefixSums.size(); ++j) {
        subarray.push_back(numCopy[j - 1]);
        if (prefixSums[j] - prefixSums[i] <= k) {
          // std::cout << "Push array: " << i << " " << j << '\n';
          subarrays.insert(subarray);
        }
      }
    }
    return subarrays.size();
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::tuple<std::vector<int>, int, int>> tests;
  tests.push_back({{2, 3, 3, 2, 2}, 2, 2});
  tests.push_back({{1, 2, 3, 4}, 4, 1});

  for (auto& [nums, k, p] : tests) {
    std::cout << s.countDistinct(nums, k, p) << '\n';
  }
}
