// link to the problem: https://leetcode.com/problems/first-missing-positive/
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
  int firstMissingPositive(std::vector<int>& nums) {
    for (auto i{0}; i < nums.size(); i++) {
      int temp = nums[i];
        while(temp > 0 && temp <= nums.size() && temp != nums[temp - 1]) {
            int replaced_value = nums[temp - 1];
            nums[temp - 1] = temp;
            temp = replaced_value;
        }
    }
    for (auto i{0}; i < nums.size(); i++) {
      if (nums[i] != i + 1) return i + 1;
    }
    return nums.size() + 1;
  }
};


int main(int argc, char** argv) {}
