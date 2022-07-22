// link to the problem: https://leetcode.com/problems/reverse-pairs/
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

// Runtime: 699 ms, faster than 59.68% of C++ online submissions for Reverse Pairs.
// Memory Usage: 76 MB, less than 66.09% of C++ online submissions for Reverse Pairs.
// This was aweful and at the end I had a bug with modulo:
// * the modulo of negative number % 2 can be -1, which is not what I expected.

constexpr bool debug = true;

template <typename T>
class BinaryIndexedTree {
 public:
  BinaryIndexedTree(std::size_t size) : size(size) { bitArray.resize(size + 1, 0); }
  void add(std::size_t index, T value) {
    ++index;
    while (index <= size) {
      bitArray[index] += value;
      index += (index & -index);
    }
  }
  T sum(std::size_t index) {
    T sum = 0;
    while (index > 0) {
      sum += bitArray[index];
      index -= (index & -index);
    }

    return sum;
  }

  T sum(std::size_t left, std::size_t right) { return sum(right) - sum(left); }

  void print(std::ostream& os) {
    std::copy(bitArray.begin(), bitArray.end(), std::ostream_iterator<T>(os, " "));
    os << '\n';
  }

 private:
  std::vector<T> bitArray;
  std::size_t size;
};

class Solution {
 public:
  int reversePairs(std::vector<int>& nums) {
    BinaryIndexedTree<int> bit(nums.size() + 1);
    int32_t answer = 0;

    std::vector<int> sortedIndexes(nums.size());
    std::iota(sortedIndexes.begin(), sortedIndexes.end(), 0);
    std::sort(sortedIndexes.begin(), sortedIndexes.end(),
              [&nums](std::size_t i1, std::size_t i2) { return nums[i1] < nums[i2]; });

    std::unordered_map<int, int> indexToSortedIndexMap;
    for (std::size_t i = 0; i < sortedIndexes.size(); ++i) {
      indexToSortedIndexMap[sortedIndexes[i]] = i;
    }

    for (int i = nums.size() - 1; i >= 0; --i) {
      const int searchedValue = (nums[i] >> 1) + (nums[i] % 2 == 1 || nums[i] % 2 == -1);
      const auto& greaterEqualNum =
          std::lower_bound(sortedIndexes.begin(), sortedIndexes.end(), searchedValue,
                           [&nums](const auto& element, const auto& value) { return nums[element] < value; });
      answer += bit.sum(std::distance(sortedIndexes.begin(), greaterEqualNum));
      bit.add(indexToSortedIndexMap[i], 1);
    }

    return answer;
  }
};
