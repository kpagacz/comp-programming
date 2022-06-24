// link to the problem: https://leetcode.com/problems/maximum-xor-with-an-element-from-array/submissions/
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

class Solution {
public:
    int findKthLargest(std::vector<int>& nums, int k) {
      std::priority_queue<int, std::vector<int>, std::greater<int>> kthLargest;
      for (const auto& num : nums) {
        kthLargest.push(num);
        if (kthLargest.size() > k) kthLargest.pop();
      }
      return kthLargest.top();
    }
};

int main(int argc, char** argv) {}
