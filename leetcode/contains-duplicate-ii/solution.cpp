// link to the problem: https://leetcode.com/problems/contains-duplicate-ii/
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
  bool containsNearbyDuplicate(std::vector<int>& nums, int k) {
    std::unordered_map<int, std::set<int>> positions;
    for (int i = 0; i < nums.size(); ++i) {
      if (positions.count(nums[i]) > 0) {
        const auto& lowerBound = positions[nums[i]].lower_bound(i - k);
        if (lowerBound != positions[nums[i]].end() && *lowerBound - i <= k) return true;
      }
      positions[nums[i]].insert(i);
    }
    return false;
  }
};

int main(int argc, char** argv) {}
