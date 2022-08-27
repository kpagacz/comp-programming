// link to the problem: https://leetcode.com/problems/find-the-k-sum-of-an-array/
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

// The trick here is understanding that lines 42 and 43 will create all possible subsequences of nums.
// Once you have that the rest is somewhat manageable - you make a djikstra search on all possible
// smallest sums to find the kth smallest.

// I did not solve this on my own...
// I figured out on my own that I need to calculate the biggest sum and then somehow create the smaller sums
// from the bigger sum, but I could not figure out how to do it. The lines in the `if` statement are the answer.

// Runtime: 252 ms, faster than 88.89% of C++ online submissions for Find the K-Sum of an Array.
// Memory Usage: 64.2 MB, less than 55.56% of C++ online submissions for Find the K-Sum of an Array.

class Solution {
 public:
  long long kSum(std::vector<int>& nums, int k) {
    std::priority_queue<std::pair<long long, int>> sums;  // sum, index of the last element in the sum
    std::sort(nums.begin(), nums.end(),
              [](const auto& first, const auto& second) { return std::abs(first) < std::abs(second); });
    long long answer = std::accumulate(nums.begin(), nums.end(), 0LL,
                                       [](long long sum, int element) { return element > 0 ? sum + element : sum; });
    sums.push({answer - std::abs(nums[0]), 0});
    while (--k) {
      const auto [sum, lastIndex] = sums.top();
      sums.pop();
      if (lastIndex + 1 < nums.size()) {
        sums.push({sum - std::abs(nums[lastIndex + 1]), lastIndex + 1});
        sums.push({sum + std::abs(nums[lastIndex]) - std::abs(nums[lastIndex + 1]), lastIndex + 1});
      }
      answer = sum;
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
