// link to the problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
public:
    int maxProfit(std::vector<int>& prices) {
      int min_price = 1000, max_profit = 0;
      for(const auto& price : prices) {
        if (price < min_price)
          min_price = price;
          else if (price - min_price > max_profit)
            max_profit = price - min_price;
      }

      return max_profit;
    }
};

int main(int argc, char** argv) {}
