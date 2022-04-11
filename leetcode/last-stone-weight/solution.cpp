#include <iostream>
#include <queue>
#include <vector>

class Solution {
 public:
  int lastStoneWeight(std::vector<int>& stones) {
    std::priority_queue<int> maxHeap{std::less<int>(), stones};
    while (maxHeap.size() > 1) {
      int first = maxHeap.top();
      maxHeap.pop();
      int second = maxHeap.top();
      maxHeap.pop();
      if (first != second) maxHeap.push(std::abs(first - second));
    }

    if (maxHeap.empty()) return 0; else
      return maxHeap.top();
  }
};
