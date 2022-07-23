// link to the problem: https://leetcode.com/problems/count-of-range-sum/
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

// I had a bug where std::partial_sum would overflow when the sum was lower than INT32_MIN.
//
// Turns out that std::partial_sum declared like this:
//    template< class InputIt, class OutputIt >
//    OutputIt partial_sum( InputIt first, InputIt last, OutputIt d_first );
// uses
//    typename std::iterator_traits<InputIt>::value_type sum = *first;
// to declare the type of the running sum.
// Obviously, in my cases this was int and not int64_t which caused the overflow.


// Lessons learned:
// * I need to remember that I can preprocess the array before tackling the problem to make it a little bit easier.

// Runtime: 501 ms, faster than 93.80% of C++ online submissions for Count of Range Sum.
// Memory Usage: 70.6 MB, less than 84.56% of C++ online submissions for Count of Range Sum.

class Solution {
 public:
  int countRangeSum(std::vector<int>& nums, int lower, int upper) {
    bit = std::vector<int>(nums.size() + 2, 0);
    std::vector<int64_t> prefixSums(nums.size() + 1, 0);
    int64_t runningSum = 0;
    for (int i = 0; i < nums.size(); ++i) {
      runningSum += nums[i];
      prefixSums[i + 1] = runningSum;
    }
    std::vector<int64_t> sortedPrefixSums = prefixSums;
    std::sort(sortedPrefixSums.begin(), sortedPrefixSums.end());

    int answer = 0;
    for (int i = prefixSums.size() - 1; i >= 0; --i) {
      const auto lowerBoundIndex =
          std::lower_bound(sortedPrefixSums.begin(), sortedPrefixSums.end(), prefixSums[i] + lower);
      const auto upperBoundIndex = std::upper_bound(lowerBoundIndex, sortedPrefixSums.end(), prefixSums[i] + upper);
      answer += sum(lowerBoundIndex - sortedPrefixSums.begin(), upperBoundIndex - sortedPrefixSums.begin());
      const auto& prefixIndex =
          std::lower_bound(sortedPrefixSums.begin(), sortedPrefixSums.end(), prefixSums[i]) - sortedPrefixSums.begin();
      add(prefixIndex, 1);
    }

    return answer;
  }

 private:
  std::vector<int> bit;
  void add(int index, int value) {
    ++index;
    while (index < bit.size()) {
      bit[index] += value;
      index += index & -index;
    }
  }

  int sum(int index) {
    int answer = 0;
    while (index > 0) {
      answer += bit[index];
      index -= index & -index;
    }
    return answer;
  }

  int sum(int left, int right) { return sum(right) - sum(left); }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::tuple<std::vector<int>, int, int>> cases{{{-2, 5, -1}, -2, 2},
                                                            // {{-3, 1, 2, 7, -5}, -3, 3},
                                                            // {{0}, 0, 0},
                                                            // {{-2, 5, -1, -7, 8, 9, 10, -11, -7, 8}, 3, 5},
                                                            {{-2147483647, 0, -2147483647, 2147483647}, -564, 3864}};
  for (auto& [nums, lower, upper] : cases) {
    std::cout << s.countRangeSum(nums, lower, upper) << "\n";
  }
}
