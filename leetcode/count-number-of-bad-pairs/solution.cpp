// link to the problem: https://leetcode.com/problems/count-number-of-bad-pairs/
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

// Runtime: 394 ms, faster than 25.00% of C++ online submissions for Count Number of Bad Pairs.
// Memory Usage: 83.6 MB, less than 100.00% of C++ online submissions for Count Number of Bad Pairs.

class Solution {
 public:
  long long countBadPairs(std::vector<int>& nums) {
    int64_t answer = 1LL * (nums.size() * (nums.size() - 1)) / 2;
    std::unordered_map<int, int> weirdMap;

    for (int i = 0; i < nums.size(); ++i) {
      if (weirdMap.count(nums[i] - i) > 0) answer -= weirdMap[nums[i] - i];
      weirdMap[nums[i] - i]++;
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
