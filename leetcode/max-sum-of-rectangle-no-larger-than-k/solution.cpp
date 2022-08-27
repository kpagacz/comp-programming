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

// Couple of things to consider:
// This is basically the same as my super bad solution except we use a set to store previuos row sums
// and use it to find a lower bound for the answer to the problem instead of calculating all possible
// rectangles and checking if they fit the solution better.
// I need to improve my intuition regarding problem statements:
// * if it asks for "something equal to K", then use a hash map
// * if it asks for "something less/greater than", then think about binary search solutions: sets, fenwick trees, etc

// Runtime: 1383 ms, faster than 75.95% of C++ online submissions for Max Sum of Rectangle No Larger Than K.
// Memory Usage: 294.1 MB, less than 14.62% of C++ online submissions for Max Sum of Rectangle No Larger Than K.

class Solution {
 public:
  int maxSumSubmatrix(std::vector<std::vector<int>>& matrix, int k) {
    int answer = INT32_MIN;
    for (int left = 0; left < matrix[0].size(); ++left) {
      std::vector<int> rowSums(matrix.size());
      for (int right = left; right < matrix[0].size(); ++right) {
        for (int i = 0; i < matrix.size(); ++i) rowSums[i] += matrix[i][right];
        std::set<int> rectangleSums{0};
        int runningRectangleSum = 0;
        for (const auto& rowSum : rowSums) {
          runningRectangleSum += rowSum;
          const auto& lowerBound = rectangleSums.lower_bound(runningRectangleSum - k);
          if (lowerBound != rectangleSums.end()) answer = std::max(answer, runningRectangleSum - *lowerBound);
          rectangleSums.insert(runningRectangleSum);
        }
      }
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  std::vector<std::pair<std::vector<std::vector<int>>, int>> tests;
  Solution s;

  tests.push_back({{{1, 0, 1}, {0, -2, 3}}, 2});
  tests.push_back({{{2, 2, -1}}, 3});
  for (auto& [matrix, k] : tests) std::cout << s.maxSumSubmatrix(matrix, k) << '\n';
}
