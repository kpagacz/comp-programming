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

    std::vector<int> sortedNums(nums);
    std::sort(sortedNums.begin(), sortedNums.end());

    for (int i = 0; i < nums.size(); ++i) {
      int index = std::upper_bound(sortedNums.begin(), sortedNums.end(), 2L * nums[i]) - sortedNums.begin();
      answer += i - bit.sum(index);
      index = std::lower_bound(sortedNums.begin(), sortedNums.end(), nums[i]) - sortedNums.begin();
      bit.add(index, 1);
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<int>> vectors{{1, 3, 2, 3, 1}, {2, 4, 3, 5, 1}, {-4, -3}, {-3, -2}};
  for (auto& nums : vectors) {
    std::copy(nums.begin(), nums.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
    std::cout << s.reversePairs(nums) << "\n";
  }
}
