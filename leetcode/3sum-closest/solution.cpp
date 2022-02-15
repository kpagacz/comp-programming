// link to the problem: https://leetcode.com/problems/3sum-closest/
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
  int twoSumCloses(const std::vector<int>& nums, int begin, const int& target) {
    int best_min = INT32_MAX;
    int best_sum = 0;
    int end = nums.size() - 1;
    while (begin < end) {
      int current_sum = nums[begin] + nums[end];
      if (std::abs(target - current_sum) < best_min) {
        best_sum = current_sum;
        best_min = std::abs(target - current_sum);
      }
      if (current_sum > target)
        end--;
      else
        begin++;
    }
    return best_sum;
  }
  int threeSumClosest(std::vector<int>& nums, int target) {
    std::sort(nums.begin(), nums.end());
    int answer;
    int best_min = INT32_MAX;
    for (auto i{0}; i < nums.size() - 2; i++) {
      int two_sum_target = target - nums[i];
      auto best_two_sum = twoSumCloses(nums, i + 1, two_sum_target);
      auto current_sum = best_two_sum + nums[i];
      if (std::abs(current_sum - target) < best_min) {
        answer = current_sum;
        best_min = std::abs(current_sum - target);
      }
    }
    return answer;
  }
};
int main(int argc, char** argv) {
  Solution s;
  std::vector<int> nums{-4, -1, 1, 2};
  std::cout << s.twoSumCloses(nums, 2, 2) << '\n';
  std::cout << s.threeSumClosest(nums, 1);
}
