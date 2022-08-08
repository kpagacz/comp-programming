// link to the problem: https://leetcode.com/problems/number-of-arithmetic-triplets/
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
  int arithmeticTriplets(std::vector<int>& nums, int diff) {
    int answer = 0;
    for (int i = 0; i < nums.size(); ++i)
      for (int j = i + 1; j < nums.size(); ++j)
        for (int k = j + 1; k < nums.size(); ++k) {
          if (nums[k] - nums[j] == diff && nums[j] - nums[i] == diff) ++answer;
        }
    return answer;
  }
};

int main(int argc, char** argv) {}
