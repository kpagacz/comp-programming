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

// Runtime: 47 ms, faster than 87.48% of C++ online submissions for Construct Target Array With Multiple Sums.
// Memory Usage: 28.7 MB, less than 77.98% of C++ online submissions for Construct Target Array With Multiple Sums.

class Solution {
 public:
  bool isPossible(std::vector<int>& target) {
    int64_t total = std::accumulate(target.begin(), target.end(),  int64_t{0});
    std::priority_queue<int, std::vector<int>> largestElementHeap(target.begin(), target.end());

    while (!largestElementHeap.empty()) {
      if (largestElementHeap.top() == 1) {
        return true;
      } else if (2 * largestElementHeap.top() - total < 1 || largestElementHeap.size() == 1)
        return false;
      else {
        int newElement = largestElementHeap.top() % (total - largestElementHeap.top());
        if (newElement == 0) newElement = total - largestElementHeap.top();
        total += newElement - largestElementHeap.top();
        largestElementHeap.pop();
        largestElementHeap.push(newElement);
      }
    }

    return true;
  }
};

int main(int argc, char** argv) {}
