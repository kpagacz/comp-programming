// link to the problem: https://leetcode.com/problems/build-array-from-permutation/
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
  std::vector<int> buildArray(std::vector<int>& nums) {
    std::vector<int> answer(nums.size());
    for (int i = 0; i < nums.size(); i++) {
      answer[i] = nums[nums[i]];
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
