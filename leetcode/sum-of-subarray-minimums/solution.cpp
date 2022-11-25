// link to the problem: https://leetcode.com/problems/sum-of-subarray-minimums/
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

// Runtime: 140 ms, faster than 77.18% of C++ online submissions for Sum of Subarray Minimums.
// Memory Usage: 39.3 MB, less than 98.84% of C++ online submissions for Sum of Subarray Minimums.

constexpr int MODULO = 1e9 + 7;
class Solution {
 public:
  int sumSubarrayMins(std::vector<int>& arr) {
    std::stack<int> increasingStack;
    int64_t sumOfMinimums = 0;

    for (int i = 0; i <= arr.size(); ++i) {
      while (!increasingStack.empty() && (i == arr.size() || arr[increasingStack.top()] >= arr[i])) {
        auto mid = increasingStack.top();
        increasingStack.pop();
        int previousLower = increasingStack.empty() ? -1 : increasingStack.top();
        int64_t count = (i - mid) * (mid - previousLower);
        count %= MODULO;
        sumOfMinimums += arr[mid] * count;
        sumOfMinimums %= MODULO;
      }
      increasingStack.push(i);
    }

    return (int)sumOfMinimums;
  }
};

int main(int argc, char** argv) {}
