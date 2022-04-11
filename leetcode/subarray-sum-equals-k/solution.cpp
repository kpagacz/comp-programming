// link to the problem: https://leetcode.com/problems/subarray-sum-equals-k/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
 public:
  int subarraySum(std::vector<int>& nums, int k) {
    std::unordered_map<int, int> prefix_sums;
    int running_sum = 0;
    prefix_sums.insert({0, 1});
    int answer = 0;
    for (auto i{0}; i < nums.size(); i++) {
      running_sum += nums[i];
      auto found = prefix_sums.find(running_sum - k);
      if (found != prefix_sums.end()) answer += found->second;
      auto inserted = prefix_sums.insert({running_sum, 1});
      if (!inserted.second) {
        inserted.first->second++;
      }
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> nums{-1,-1,1};
  int k = 1;
  std::cout << s.subarraySum(nums, k) << '\n';
}
