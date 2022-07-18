// link to the problem: https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/
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

// Runtime: 1171 ms, faster than 51.25% of C++ online submissions for Number of Submatrices That Sum to Target.
// Memory Usage: 168.1 MB, less than 39.35% of C++ online submissions for Number of Submatrices That Sum to Target.

class Solution {
 public:
  int numSubmatrixSumTarget(std::vector<std::vector<int>>& matrix, int target) {
    std::vector<std::vector<int>> rowPrefixSums(matrix.size(), std::vector<int>(matrix[0].size() + 1, 0));
    int answer = 0;
    for (int i = 0; i < matrix.size(); ++i)
      std::partial_sum(matrix[i].begin(), matrix[i].end(), rowPrefixSums[i].begin() + 1);

    // print rowPrefixSums
    // for (int i = 0; i < rowPrefixSums.size(); ++i) {
    //   for (int j = 0; j < rowPrefixSums[i].size(); ++j) std::cout << rowPrefixSums[i][j] << " ";
    //   std::cout << std::endl;
    // }

    for (int i = 0; i < rowPrefixSums[0].size(); ++i)
      for (int j = i + 1; j < rowPrefixSums[0].size(); ++j) {
        std::vector<int> rowSum;
        std::transform(rowPrefixSums.begin(), rowPrefixSums.end(), std::back_inserter(rowSum),
                       [&](const std::vector<int>& row) { return row[j] - row[i]; });
        // std::copy(rowSum.begin(), rowSum.end(), std::ostream_iterator<int>(std::cout, " "));
        // std::cout << '\n';
        answer += sumToTarget1D(rowSum, target);
      }
    return answer;
  }

 private:
  int sumToTarget1D(const std::vector<int>& nums, const int& target) {
    std::unordered_map<int, int> prefixSums(nums.size());
    std::vector<int> prefixSumsVec(nums.size() + 1, 0);
    std::partial_sum(nums.begin(), nums.end(), prefixSumsVec.begin() + 1);
    int count = 0;
    for (const auto& prefixSum : prefixSumsVec) {
      if (prefixSums.count(prefixSum - target) > 0) count += prefixSums[prefixSum - target];
      prefixSums[prefixSum]++;
    }

    return count;
  }
};

int main(int argc, char** argv) {}
