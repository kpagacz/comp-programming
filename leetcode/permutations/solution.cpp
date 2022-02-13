// link to the problem: https://leetcode.com/problems/permutations/
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
  std::vector<std::vector<int>> permute(std::vector<int>& nums) {
    std::sort(nums.begin(), nums.end());
    std::vector<std::vector<int>> answer;
    answer.push_back(nums);
    while (std::next_permutation(nums.begin(), nums.end())) answer.push_back(nums);
    return answer;
  }
};
int main(int argc, char** argv) {}
