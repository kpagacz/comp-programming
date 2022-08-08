// link to the problem: https://leetcode.com/problems/longest-increasing-subsequence/
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
  int lengthOfLIS(std::vector<int>& nums) {
    std::vector<int> lis;
    for (const auto& num : nums) {
      if (lis.empty() || lis.back() < num) {
        lis.push_back(num);
      } else {
        auto lowerBound = std::lower_bound(lis.begin(), lis.end(), num);
        *lowerBound = num;
      }
    }
    return lis.size();
  }
};

int main(int argc, char** argv) { std::vector<int> }
