// link to the problem: https://leetcode.com/problems/find-k-closest-elements/
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

// Runtime: 119 ms, faster than 44.99% of C++ online submissions for Find K Closest Elements.
// Memory Usage: 31.9 MB, less than 61.49% of C++ online submissions for Find K Closest Elements.

class Solution {
 public:
  std::vector<int> findClosestElements(std::vector<int>& arr, int k, int x) {
    std::vector<int> answer;
    auto lowerBound = std::lower_bound(arr.begin(), arr.end(), x) - arr.begin();
    // std::cout << lowerBound << '\n';
    if (lowerBound == arr.size()) lowerBound--;
    int closest = lowerBound;
    if (lowerBound > 0 && std::abs(arr[lowerBound - 1] - x) <= std::abs(arr[lowerBound] - x)) closest = lowerBound - 1;
    // std::cout << closest << "\n";

    int backwards = closest, forwards = closest;
    k--;
    while (k) {
      if (backwards == 0) {
        forwards++;
        k--;
        continue;
      }
      if (forwards == arr.size() - 1) {
        backwards--;
        k--;
        continue;
      }
      if (std::abs(x - arr[backwards - 1]) <= std::abs(x - arr[forwards + 1]))
        backwards--;
      else
        forwards++;
      k--;
    }

    while (backwards <= forwards) answer.push_back(arr[backwards++]);
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::tuple<std::vector<int>, int, int>> cases;
  cases.push_back({{1, 3, 6}, 1, 3});
  cases.push_back({{1, 3, 6}, 1, 10});
  cases.push_back({{1, 3, 6}, 1, -1});
  cases.push_back({{1, 3, 6}, 1, 4});
  cases.push_back({{1, 3, 6}, 1, 5});
  for (auto [arr, k, x] : cases) {
    auto answer = s.findClosestElements(arr, k, x);
    std::copy(answer.begin(), answer.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
  }
}
