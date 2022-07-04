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

// It is a quadratic solution and expectedly times out.
// This times out.

class Solution {
 public:
  int candy(std::vector<int>& ratings) {
    std::vector<int> answer(ratings.size(), 1);
    bool changedRating = false;
    do {
      changedRating = false;
      for (int i = 0; i < ratings.size(); ++i) {
        if (i != 0 && ratings[i - 1] < ratings[i] && answer[i - 1] >= answer[i]) {
          answer[i] = answer[i - 1] + 1;
          changedRating = true;
        }
        if (i != ratings.size() - 1 && ratings[i + 1] < ratings[i] && answer[i + 1] >= answer[i]) {
          answer[i] = answer[i + 1] + 1;
          changedRating = true;
        }
      }
      std::copy(answer.begin(), answer.end(), std::ostream_iterator<int>(std::cout, " "));
      std::cout << '\n';
    } while (changedRating);

    return std::accumulate(answer.begin(), answer.end(), 0);
  }
};

int main(int argc, char** argv) {}
