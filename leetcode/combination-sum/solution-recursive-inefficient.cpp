// link to the problem: https://leetcode.com/problems/combination-sum/
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
  void recursiveKnapsack(std::vector<std::vector<int>>& answers, const int& target, std::vector<int> added, const std::vector<int> candidates) {
    if (target < 0) return;
    if (target == 0) {
      answers.push_back(added);
      return;
    }
    for (auto num = candidates.begin(); num != candidates.end(); num++) {
      added.push_back(*num);
      recursiveKnapsack(answers, target - *num, added, std::vector<int>(num, candidates.end()));
      added.pop_back();
    }
  }
  std::vector<std::vector<int>> combinationSum(std::vector<int>& candidates, int target) {
    std::vector<std::vector<int>> answer;
    recursiveKnapsack(answer, target, std::vector<int>(), candidates);
    return answer;
  }
};

int main(int argc, char** argv) {}
