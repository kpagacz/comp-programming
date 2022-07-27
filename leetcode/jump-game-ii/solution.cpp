// link to the problem: https://leetcode.com/problems/jump-game-ii/
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
  int jump(std::vector<int>& nums) {
    std::vector<int> minJumps(nums.size(), INT32_MAX);
    minJumps[0] = 0;
    for (int i = 0; i < minJumps.size(); ++i) {
      for (int j = 1; j <= nums[i] && i + j < minJumps.size(); ++j)
        minJumps[i + j] = std::min(minJumps[i + j], minJumps[i] + 1);
    }
    return minJumps.back();
  }

  int jumpQuick(std::vector<int>& nums) {
    int jumps = 0, currentEnd = 0, farthest = 0;
    for (int i = 0; i < nums.size() - 1; ++i) {
      farthest = std::max(farthest, i + nums[i]);
      if (i == currentEnd) {
        ++jumps;
        currentEnd = farthest;
      }
    }

    return jumps;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> v{1, 2, 1, 1, 1};
  std::cout << s.jumpQuick(v);
}
