// link to the problem: https://leetcode.com/problems/sum-of-even-numbers-after-queries/
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

// Runtime: 275 ms, faster than 13.69% of C++ online submissions for Sum of Even Numbers After Queries.
// Memory Usage: 45.4 MB, less than 90.67% of C++ online submissions for Sum of Even Numbers After Queries.

class Solution {
 public:
  std::vector<int> sumEvenAfterQueries(std::vector<int>& nums, std::vector<std::vector<int>>& queries) {
    int sum = std::accumulate(nums.begin(), nums.end(), 0,
                              [](const auto& sum, const auto& el) { return el % 2 ? sum : sum + el; });
    std::vector<int> answer;
    answer.reserve(queries.size());
    for (const auto& query : queries) {
      const int &index = query[1], &val = query[0];
      if (nums[index] % 2 == 0) sum -= nums[index];
      nums[index] += val;
      if (nums[index] % 2 == 0) sum += nums[index];
      answer.push_back(sum);
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
