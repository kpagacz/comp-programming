// link to the problem: https://leetcode.com/problems/range-product-queries-of-powers/
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

// Runtime: 433 ms, faster than 95.59% of C++ online submissions for Range Product Queries of Powers.
// Memory Usage: 140.2 MB, less than 91.10% of C++ online submissions for Range Product Queries of Powers.

class Solution {
 public:
  std::vector<int> productQueries(int n, std::vector<std::vector<int>>& queries) {
    std::vector<int> answer;
    answer.reserve(queries.size());

    std::vector<int> indicesWithSetBit;
    for (int i = 0; i < 32; i++)
      if (n & (1 << i)) indicesWithSetBit.push_back(i);

    for (const auto& query : queries) {
      int64_t product = 1;
      for (int position = query[0]; position <= query[1]; position++) {
        product = (product * (1 << indicesWithSetBit[position])) % MODULO;
      }
      answer.push_back(product);
    }

    return answer;
  }
  const int MODULO = 1e9 + 7;
};

int main(int argc, char** argv) {}
