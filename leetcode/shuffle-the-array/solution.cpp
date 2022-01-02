// link to the problem: https://leetcode.com/problems/shuffle-the-array/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <string>
#include <vector>

class Solution {
 public:
  std::vector<int> shuffle(std::vector<int>& nums, int n) {
    std::vector<int> answer(nums.size());
    for(int i{0}; i < answer.size(); i++) {
      if (i % 2) {
        answer[i] = nums[n + i / 2];
      } else {
        answer[i] = nums[i / 2];
      }
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
