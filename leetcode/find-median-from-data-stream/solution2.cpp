// link to the problem: https://leetcode.com/problems/find-median-from-data-stream/
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

// Runtime: 403 ms, faster than 82.92% of C++ online submissions for Find Median from Data Stream.
// Memory Usage: 117.1 MB, less than 65.05% of C++ online submissions for Find Median from Data Stream.

class MedianFinder {
  std::priority_queue<int> lowerHalf{std::less<int>()};
  std::priority_queue<int, std::vector<int>, std::greater<int>> greaterHalf{std::greater<int>()};

 public:
  MedianFinder() {}

  void addNum(int num) {
    lowerHalf.push(num);
    greaterHalf.push(lowerHalf.top());
    lowerHalf.pop();

    if (greaterHalf.size() > lowerHalf.size()) lowerHalf.push(greaterHalf.top()), greaterHalf.pop();
  }

  double findMedian() {
    if (lowerHalf.size() == greaterHalf.size()) return 1.0 * (lowerHalf.top() + greaterHalf.top()) / 2.0;
    return lowerHalf.top();
  }
};

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder* obj = new MedianFinder();
 * obj->addNum(num);
 * double param_2 = obj->findMedian();
 */

int main(int argc, char** argv) {
  MedianFinder s;
}
