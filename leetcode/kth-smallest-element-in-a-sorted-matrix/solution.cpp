// link to the problem: https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
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

struct Comparer {
  bool operator()(const std::pair<int, std::pair<int, int>>& first, const std::pair<int, std::pair<int, int>>& second) {
    return first.first > second.first;
  }
};

class Solution {
 public:
  int kthSmallest(std::vector<std::vector<int>>& matrix, int k) {
    std::priority_queue<std::pair<int, std::pair<int, int>>, std::vector<std::pair<int, std::pair<int, int>>>, Comparer>
        heap;
    for (int i = 0; i < matrix[0].size(); ++i) heap.push({matrix[0][i], {0, i}});
    while (--k) {
      const auto top = heap.top();
      heap.pop();

      if (top.second.first + 1 < matrix.size()) {
        heap.push({matrix[top.second.first + 1][top.second.second], {top.second.first + 1, top.second.second}});
      }
    }

    return heap.top().first;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<int>> v{{1, 5, 9}, {10, 11, 13}};
  std::cout << s.kthSmallest(v, 4) << '\n';
}
