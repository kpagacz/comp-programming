// link to the problem: https://leetcode.com/problems/construct-target-array-with-multiple-sums/
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

// This was my first attempt at a solution. It passes but is very slow.
// Runtime: 102 ms, faster than 5.99% of C++ online submissions for Construct Target Array With Multiple Sums.
// Memory Usage: 29.5 MB, less than 64.96% of C++ online submissions for Construct Target Array With Multiple Sums.

class Solution {
 public:
  bool isPossible(std::vector<int>& target) {
    int64_t total = std::accumulate(target.begin(), target.end(),  int64_t{0});
    std::priority_queue<int64_t, std::vector<int64_t>> largestElementHeap(target.begin(), target.end());

    while (!largestElementHeap.empty()) {
      if (largestElementHeap.top() == 1) {
        total -= 1;
        largestElementHeap.pop();
        continue;
      } else if (2 * largestElementHeap.top() - total < 1)
        return false;
      else if (largestElementHeap.size() == 1)
        return false;
      else {
        int64_t topDecrement = total - largestElementHeap.top(); // 1
        int64_t oldTop = largestElementHeap.top(); // 10
        largestElementHeap.pop();

        int64_t difference = oldTop - largestElementHeap.top(); // 10 - 1 = 9
        oldTop -= topDecrement * (difference / topDecrement); // 1 * (9 / 1) -> 10 - 9 = 1
        if (oldTop > largestElementHeap.top()) oldTop -= topDecrement;
        total = topDecrement + oldTop;
        largestElementHeap.push(oldTop);
      }
    }

    return true;
  }
};

int main(int argc, char** argv) {}
