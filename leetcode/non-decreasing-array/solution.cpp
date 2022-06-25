// link to the problem: https://leetcode.com/problems/non-decreasing-array/
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
  bool checkPossibility(std::vector<int>& nums) {
    auto decreasing = std::adjacent_find(nums.begin(), nums.end(), std::greater<int>());
    if (decreasing == nums.end()) return true;  // no decreasing occurences
    if (std::adjacent_find(decreasing + 1, nums.end(), std::greater<int>()) != nums.end())
      return false;  // there is a second decreasing occurenace

    // two options that let us modify the array only once for the non-decreasing condition:
    // * there are no greater elements than the second element from the adjacent pair before the first element
    // * there are no lower elements than the first element from the adjacent pair after the second element
    auto greaterThanSecondFromPair = [&](auto& el) { return el > *(decreasing + 1); };
    auto findBeforeFirst = std::find_if(nums.begin(), decreasing, greaterThanSecondFromPair);
    auto findAfterSecond =
        std::find_if(std::next(decreasing, 2), nums.end(), [&](auto& el) { return el < *decreasing; });
    return findBeforeFirst == decreasing || findAfterSecond == nums.end();
  }
};

int main(int argc, char** argv) {}
