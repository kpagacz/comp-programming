// link to the problem: https://leetcode.com/problems/count-of-smaller-numbers-after-self/
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
  std::vector<int> countSmaller(std::vector<int>& nums) {
    bit = std::vector<int>(nums.size() + 1, 0);
    std::vector<int> sortedNums = nums;
    std::sort(sortedNums.begin(), sortedNums.end());

    std::vector<int> counts(nums.size(), 0);
    for (int i = nums.size() - 1; i >= 0; --i) {
      const auto sortedIndex = std::lower_bound(sortedNums.begin(), sortedNums.end(), nums[i]) - sortedNums.begin();
      counts[i] = sum(sortedIndex);
      add(sortedIndex, 1);
    }
    return counts;
  }

 private:
  std::vector<int> bit;
  int sum(int i) {
    int answer = 0;
    while (i > 0) {
      answer += bit[i];
      i -= i & -i;
    }
    return answer;
  }

  void add(int index, int value) {
    ++index;
    while (index < bit.size()) {
      bit[index] += value;
      index += index & -index;
    }
  }
};

int main(int argc, char** argv) {}
