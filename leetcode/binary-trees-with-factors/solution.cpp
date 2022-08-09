// link to the problem: https://leetcode.com/problems/binary-trees-with-factors/
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

// Runtime: 132 ms, faster than 41.54% of C++ online submissions for Binary Trees With Factors.
// Memory Usage: 9.2 MB, less than 81.85% of C++ online submissions for Binary Trees With Factors.

constexpr int MODULO = 1e9 + 7;

class Solution {
 public:
  int numFactoredBinaryTrees(std::vector<int>& arr) {
    std::sort(arr.begin(), arr.end());

    std::unordered_map<int, int> numbersMap;
    for (int i = 0; i < arr.size(); ++i) numbersMap.insert({arr[i], i});

    std::vector<int64_t> treesCount(arr.size(), 1);

    for (int i = 0; i < arr.size(); ++i) {
      for (int j = 0; j < i; ++j) {
        if (arr[i] % arr[j] == 0) {
          int right = arr[i] / arr[j];
          if (numbersMap.count(right) > 0)
            treesCount[i] = (treesCount[i] + treesCount[j] * treesCount[numbersMap[right]]) % MODULO;
        }
      }
    }

    int64_t answer = 0;
    for (const auto& count : treesCount) answer = (answer + count) % MODULO;
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<int>> cases{{2, 4}, {2, 4, 5, 10}, {10, 4, 5, 2}};
  for (auto& test : cases) {
    std::cout << s.numFactoredBinaryTrees(test);
    std::cout << '\n';
  }
}
