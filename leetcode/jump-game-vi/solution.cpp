// link to the problem: https://leetcode.com/problems/jump-game-vi/
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
  int maxResult(std::vector<int>& nums, int k) {
    using State = std::pair<int, int>;  // index, cost
    auto stateComparator = [](const auto& first, const auto& second) {
      return std::get<1>(first) < std::get<1>(second);
    };
    std::priority_queue<State, std::vector<State>, decltype(stateComparator)> heap{stateComparator};
    std::vector<int> costs(nums.size());

    for (int i = 0; i < nums.size(); ++i) {
      while (!heap.empty() && heap.top().first < i - k) heap.pop();
      costs[i] = nums[i];
      costs[i] += heap.empty() ? 0 : heap.top().second;
      heap.push({i, costs[i]});
    }
    return costs[nums.size() - 1];
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> v{1, -1, -2, 4, -7, 3};
  int k = 2;
  std::cout << s.maxResult(v, k);
}
