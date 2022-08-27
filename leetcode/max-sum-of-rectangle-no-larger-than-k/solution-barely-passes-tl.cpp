// link to the problem: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/
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

// Runtime: 2454 ms, faster than 8.40% of C++ online submissions for Max Sum of Rectangle No Larger Than K.
// Memory Usage: 11 MB, less than 94.12% of C++ online submissions for Max Sum of Rectangle No Larger Than K.

class Solution {
 public:
  int maxSumSubmatrix(std::vector<std::vector<int>>& matrix, int k) {
    std::vector<std::vector<int>> rowPrefixSums(matrix.size(), std::vector<int>(matrix[0].size() + 1));
    for (int i = 0; i < matrix.size(); ++i) {
      rowPrefixSums[i][0] = 0;
      std::partial_sum(matrix[i].begin(), matrix[i].end(), rowPrefixSums[i].begin() + 1);
    }
    // for (const auto& prefixSums : rowPrefixSums) {
    //   std::copy(prefixSums.begin(), prefixSums.end(), std::ostream_iterator<int>(std::cout, " "));
    //   std::cout << "\n";
    // }
    int answer = INT32_MIN;
    for (int i = 0; i < matrix.size(); ++i)
      for (int j = 0; j < matrix[0].size(); ++j)
        for (int col = j + 1; col <= matrix[0].size(); ++col) {
          int sum = 0;
          for (int row = i; row < matrix.size(); ++row) {
            sum += rowPrefixSums[row][col] - rowPrefixSums[row][j];
            // std::cout << "Coordinates: " << i << " " << j << " row size: " << row - i << " col size: " << col - j
            //           << '\n';
            // std::cout << "Added sum: " << sum << '\n';
            if (sum > answer && sum <= k) answer = sum;
          }
        }

    return answer;
  }
};

int main(int argc, char** argv) {
  std::vector<std::pair<std::vector<std::vector<int>>, int>> tests;
  Solution s;

  tests.push_back({{{1, 0, 1}, {0, -2, 3}}, 2});
  for (auto& [matrix, k] : tests) std::cout << s.maxSumSubmatrix(matrix, k) << '\n';
}
