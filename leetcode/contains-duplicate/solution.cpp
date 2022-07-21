// link to the problem: https://leetcode.com/problems/contains-duplicate/
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
  bool containsDuplicate(std::vector<int>& nums) {
    return nums.size() != std::unordered_set<int>(nums.begin(), nums.end()).size();
  }
};

int main(int argc, char** argv) {}
