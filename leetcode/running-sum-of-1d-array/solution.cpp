// link to the problem: https://leetcode.com/problems/running-sum-of-1d-array/
#include<iostream>
#include<vector>
#include<array>
#include<string>
#include<algorithm>
#include<numeric>
#include<sstream>
#include<iterator>
#include<queue>

class Solution {
public:
    std::vector<int> runningSum(std::vector<int>& nums) {
      for (int i{1}; i < nums.size(); i++) nums[i] = nums[i] + nums[i - 1];
      return nums;
    }
};

int main(int argc, char** argv) {

}
