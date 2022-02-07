// link to the problem: https://leetcode.com/problems/3sum/
// Here I was struggling a lot with an address sanitizer. I was missing a heap buffer overflow bug somewhere in the
// code.
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
  std::vector<std::array<int, 2>> twoSumAnswers;
  void twoSum(const std::vector<int>& nums, int left, int right, const int& target) {
    twoSumAnswers.clear();
    while (left < right) {
      if (nums[left] + nums[right] == target) {
        twoSumAnswers.push_back({left, right});
        while (left < right && nums[left] == nums[left + 1]) left++;
        left++;
      } else if (nums[left] + nums[right] < target) {
        while (left < right && nums[left] == nums[left + 1]) left++;
        left++;
      } else {
        while (left < right && nums[right] == nums[right - 1]) right--;
        right--;
      }
    }
  }

  std::vector<std::vector<int>> threeSum(std::vector<int>& nums) {
    std::vector<std::vector<int>> answer;
    std::sort(nums.begin(), nums.end());

    for (int i = 0; i < nums.size(); i++) {
      int target = nums[i], left = i + 1, right = nums.size() - 1;
      twoSum(nums, left, right, -target);
      for (const auto& pair : twoSumAnswers) {
        answer.push_back({nums[i], nums[pair[0]], nums[pair[1]]});
      }
      while (i < nums.size() - 1 && nums[i] == nums[i + 1]) i++;
      // I had an overflow here: i < nums.size() insead of i < nums.size() - 1
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> nums{1, 2, 3, 4, 2};
  auto ans = s.threeSum(nums);
  std::cout << "Answers\n";
  for (const auto& values : ans) {
    std::copy(values.begin(), values.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
  }
}
