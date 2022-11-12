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

// Runtime: 328 ms, faster than 98.14% of C++ online submissions for Find Median from Data Stream.
// Memory Usage: 116.9 MB, less than 98.02% of C++ online submissions for Find Median from Data Stream.

class MedianFinder {
  std::priority_queue<int> lowerHalf{std::less<int>()};
  std::priority_queue<int, std::vector<int>, std::greater<int>> greaterHalf{std::greater<int>()};

 public:
  MedianFinder() {}

  void addNum(int num) {
    if (lowerHalf.empty() && greaterHalf.empty()) {
      lowerHalf.push(num);
      return;
    }

    if (num > lowerHalf.top()) {
      greaterHalf.push(num);
      if (greaterHalf.size() > lowerHalf.size() + 1) {
        lowerHalf.push(greaterHalf.top());
        greaterHalf.pop();
      }
    } else {
      lowerHalf.push(num);
      if (lowerHalf.size() > greaterHalf.size() + 1) {
        greaterHalf.push(lowerHalf.top());
        lowerHalf.pop();
      }
    }
  }

  double findMedian() {
    if (lowerHalf.size() > greaterHalf.size()) return lowerHalf.top();
    else if (greaterHalf.size() > lowerHalf.size()) return greaterHalf.top();
    else return 1.0 * (greaterHalf.top() + lowerHalf.top()) / 2;
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
