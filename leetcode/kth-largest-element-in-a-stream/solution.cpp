// link to the problem: https://leetcode.com/problems/kth-largest-element-in-a-stream/
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

class KthLargest {
 public:
  std::priority_queue<int, std::vector<int>, decltype(std::greater<int>())> stream{std::greater<int>()};
  int k;
  KthLargest(int k, std::vector<int>& nums) {
    for (const auto& num : nums) stream.push(num);
    while (stream.size() > k) stream.pop();
    this->k = k;
  }

  int add(int val) {
    stream.push(val);
    while (stream.size() > k) stream.pop();
    return stream.top();
  }
};

/**
 * Your KthLargest object will be instantiated and called as such:
 * KthLargest* obj = new KthLargest(k, nums);
 * int param_1 = obj->add(val);
 */

int main(int argc, char** argv) {}
