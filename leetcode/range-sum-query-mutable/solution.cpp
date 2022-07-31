// link to the problem: https://leetcode.com/problems/range-sum-query-mutable/
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

class BinaryIndexedTree {
 public:
  BinaryIndexedTree(std::size_t size) { bit = std::vector<int>(size + 1, 0); }

  BinaryIndexedTree() = default;

  void update(std::size_t index, int value) {
    ++index;
    while (index < bit.size()) {
      bit[index] += value;
      index += (index & -index);
    }
  }

  void set(std::size_t index, int value) {
    int difference = value - bit[index + 1];
    std::copy(bit.begin(), bit.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << "\n";
    std::cout << "difference " << difference << '\n';
    update(index, difference);
  }

  int sum(std::size_t index) {
    int answer = 0;
    while (index > 0) {
      answer += bit[index];
      index -= (index & -index);
    }
    return answer;
  }

  int sum(std::size_t start, std::size_t end) { return sum(end) - sum(start); }

 private:
  std::vector<int> bit;
};

class NumArray {
 public:
  NumArray(std::vector<int>& nums) {
    bit = BinaryIndexedTree(nums.size());
    for (int i = 0; i < nums.size(); ++i) bit.update(i, nums[i]);
    this->nums = nums;
  }

  void update(int index, int val) { int difference = val - nums[index];
    nums[index] = val;
    bit.update(index, difference);
  }

  int sumRange(int left, int right) { return bit.sum(left, right + 1); }

 private:
  BinaryIndexedTree bit;
  std::vector<int> nums;
};

/**
 * Your NumArray object will be instantiated and called as such:
 * NumArray* obj = new NumArray(nums);
 * obj->update(index,val);
 * int param_2 = obj->sumRange(left,right);
 */

int main(int argc, char** argv) {
  std::vector<std::vector<int>> cases{{1, 3, 5}};

  for (auto& test : cases) {
    NumArray array(test);
    std::cout << array.sumRange(0, 2) << '\n';
    array.update(1, 2);
    std::cout << array.sumRange(0, 2) << '\n';
  }
}
