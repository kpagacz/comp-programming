// link to the problem: https://leetcode.com/problems/summary-ranges/
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
  std::vector<std::string> summaryRanges(std::vector<int>& nums) {
    if (nums.empty()) return {};
    std::vector<std::pair<int, int>> ranges;
    int first = 0, last = 0;
    for (auto i{1}; i < nums.size(); i++) {
      if (nums[i] == nums[i - 1] + 1) {
        last = i;
      } else {
        ranges.push_back({nums[first], nums[last]});
        first = last = i;
      }
    }
    ranges.push_back({nums[first], nums[last]});
    std::vector<std::string> answer;
    auto pairs_to_strings = [](const auto& pair) {
      if (pair.first == pair.second)
        return std::to_string(pair.first);
      else
        return std::to_string(pair.first) + "->" + std::to_string(pair.second);
    };
    std::transform(ranges.begin(), ranges.end(), std::back_inserter(answer), pairs_to_strings);
    return answer;
  }
};

int main(int argc, char** argv) {}
