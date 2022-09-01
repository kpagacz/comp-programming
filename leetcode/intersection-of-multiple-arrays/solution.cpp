// link to the problem: https://leetcode.com/problems/intersection-of-multiple-arrays/
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

// Runtime: 16 ms, faster than 74.99% of C++ online submissions for Intersection of Multiple Arrays.
// Memory Usage: 11.6 MB, less than 89.62% of C++ online submissions for Intersection of Multiple Arrays.

class Solution {
 public:
  std::vector<int> intersection(std::vector<std::vector<int>>& nums) {
    std::vector<int> answer, buffer;
    for (auto& array : nums) std::sort(array.begin(), array.end());
    answer = nums[0];
    for (const auto& array : nums) {
      buffer.clear();
      std::set_intersection(answer.begin(), answer.end(), array.begin(), array.end(), std::back_inserter(buffer));
      std::swap(answer, buffer);
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
