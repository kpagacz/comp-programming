// link to the problem: https://leetcode.com/problems/number-of-flowers-in-full-bloom/
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

// Runtime: 431 ms, faster than 76.22% of C++ online submissions for Number of Flowers in Full Bloom.
// Memory Usage: 78.6 MB, less than 98.96% of C++ online submissions for Number of Flowers in Full Bloom.

class Solution {
 public:
  std::vector<int> fullBloomFlowers(std::vector<std::vector<int>>& flowers, std::vector<int>& persons) {
    std::vector<int> starts, ends;
    starts.reserve(flowers.size()), ends.reserve(flowers.size());
    for (const auto& flower : flowers) starts.push_back(flower[0]), ends.push_back(flower[1]);
    std::sort(starts.begin(), starts.end());
    std::sort(ends.begin(), ends.end());

    std::vector<int> answer;
    answer.reserve(persons.size());
    for (const auto& person : persons) {
      auto flowersStarted = std::upper_bound(starts.begin(), starts.end(), person) - starts.begin();
      auto flowersEnded = std::lower_bound(ends.begin(), ends.end(), person) - ends.begin();
      answer.push_back(flowersStarted - flowersEnded);
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
