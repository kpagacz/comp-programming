// link to the problem: https://leetcode.com/problems/online-stock-span/
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

// It's a nice solution, but suprisingly slow. I think it might be the case of
// storing the pairs in a stack and using integers instead of indices.

// Runtime: 584 ms, faster than 11.01% of C++ online submissions for Online Stock Span.
// Memory Usage: 84.3 MB, less than 22.76% of C++ online submissions for Online Stock Span.

class StockSpanner {
 public:
  std::stack<std::pair<int, int>> monotonicStack;
  StockSpanner() {}

  int next(int price) {
    int span = 1;
    while (!monotonicStack.empty() && monotonicStack.top().first <= price) {
      span += monotonicStack.top().second;
      monotonicStack.pop();
    }
    monotonicStack.push({price, span});
    return span;
  }
};

/**
 * Your StockSpanner object will be instantiated and called as such:
 * StockSpanner* obj = new StockSpanner();
 * int param_1 = obj->next(price);
 */

int main(int argc, char** argv) {}
