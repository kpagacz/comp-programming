// link to the problem: https://leetcode.com/problems/combination-sum/
// What did I learn:
// * I knew from the analysis that I could do it either recursively or iteratively
// * I couldn't find an easy way to determine the array of numbers to be added to the answer iteratively, so I
//   went with the recursive approach
// * This is basically recursion with backtracking
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
  void recursiveKnapsack(std::vector<std::vector<int>>& answers, const int& target, std::vector<int>& added, const std::vector<int>& candidates, const int& candidates_it) {
    if (target < 0) return;
    if (target == 0) {
      answers.push_back(added);
      return;
    }
    for (auto i{candidates_it}; i < candidates.size(); i++) {
      added.push_back(candidates[i]);
      recursiveKnapsack(answers, target - candidates[i], added, candidates, i);
      added.pop_back(); // backtracking happens here
    }
  }
  std::vector<std::vector<int>> combinationSum(std::vector<int>& candidates, int target) {
    std::vector<std::vector<int>> answer;
    std::vector<int> added;
    recursiveKnapsack(answer, target, added, candidates, 0);
    return answer;
  }
};

int main(int argc, char** argv) {}
