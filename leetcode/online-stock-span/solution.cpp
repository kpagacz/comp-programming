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

class StockSpanner {
 public:
  std::stack<int> monotonicStack;
  std::vector<int> prices;
  StockSpanner() {}

  int next(int price) {
    prices.push_back(price);
    while (!monotonicStack.empty() && prices[monotonicStack.top()] <= price) {
      monotonicStack.pop();
    }
    int answer = prices.size();
    if (!monotonicStack.empty()) answer -= monotonicStack.top() - 1;
    monotonicStack.push(prices.size() - 1);
    return answer;
  }
};

/**
 * Your StockSpanner object will be instantiated and called as such:
 * StockSpanner* obj = new StockSpanner();
 * int param_1 = obj->next(price);
 */

int main(int argc, char** argv) {}
