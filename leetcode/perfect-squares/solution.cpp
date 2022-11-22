// link to the problem: https://leetcode.com/problems/perfect-squares/
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

// Runtime: 442 ms, faster than 37.23% of C++ online submissions for Perfect Squares.
// Memory Usage: 9.1 MB, less than 69.87% of C++ online submissions for Perfect Squares.

class Solution {
 public:
  int numSquares(int n) {
    std::vector<int> leastSquares(n + 1);
    std::iota(leastSquares.begin(), leastSquares.end(), 0);
    for (int i = 2; i <= (int)std::sqrt(n); i++) {
      for (int j = i * i; j < leastSquares.size(); j++)
        leastSquares[j] = std::min(leastSquares[j], leastSquares[j - i * i] + 1);
    }

    return leastSquares[n];
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> cases;
  cases.push_back(1);
  cases.push_back(2);
  cases.push_back(3);
  cases.push_back(16);
  cases.push_back(1000);
  cases.push_back(12);
  cases.push_back(13);
  for (auto test : cases) std::cout << s.numSquares(test) << '\n';
}
