// link to the problem: https://leetcode.com/problems/candy/
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

// This is nice.
// I had a bug here where I put `i` instead of `it` in one of the while conditions. Spent 5 minutes debugging it...
// Runtime: 28 ms, faster than 70.81% of C++ online submissions for Candy.
// Memory Usage: 17.6 MB, less than 84.26% of C++ online submissions for Candy.

class Solution {
 public:
  int candy(std::vector<int>& ratings) {
    // std::copy(ratings.begin(), ratings.end(), std::ostream_iterator<int>(std::cout, " "));
    // std::cout << "\n";
    std::vector<int> answer(ratings.size(), 1);
    if (ratings.size() == 1) return 1;
    for (std::size_t i = 0; i < ratings.size(); ++i) {
      const auto leftBorder = i == 0;
      const auto rightBorder = i == ratings.size() - 1;
      // check for local minima
      if (leftBorder || rightBorder || (ratings[i - 1] >= ratings[i] && ratings[i + 1] >= ratings[i])) {
        std::size_t it = i;
        int additionalApples = 2;
        while (it != 0 && ratings[it - 1] > ratings[it]) {
          answer[it - 1] = std::max(answer[it - 1], additionalApples);
          ++additionalApples;
          --it;
        }
        // std::copy(answer.begin(), answer.end(), std::ostream_iterator<int>(std::cout, " "));
        // std::cout << '\n';

        additionalApples = 2;
        it = i;
        while (it != ratings.size() - 1 && ratings[it + 1] > ratings[it]) {
          answer[it + 1] = std::max(answer[it + 1], additionalApples);
          ++it;
          ++additionalApples;
        }
        // std::copy(answer.begin(), answer.end(), std::ostream_iterator<int>(std::cout, " "));
        // std::cout << '\n';
      }
    }
    return std::accumulate(answer.begin(), answer.end(), 0);
  }
};

int main() {
  Solution s;
  std::vector<int> v{1, 2, 2};
  s.candy(v);
}
