// link to the problem: https://leetcode.com/problems/combination-sum-ii/
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

// Runtime: 7 ms, faster than 76.01% of C++ online submissions for Combination Sum II.
// Memory Usage: 11.3 MB, less than 32.21% of C++ online submissions for Combination Sum II.

class Solution {
 public:
  std::vector<std::vector<int>> combinationSum2(std::vector<int>& candidates, int target) {
    std::vector<std::vector<int>> answer;
    std::unordered_map<int, int> frequencies;
    std::vector<int> added;
    for (const auto& candidate : candidates) frequencies[candidate]++;
    recursiveKnapsack(frequencies.begin(), answer, added, target, frequencies.end());
    return answer;
  }

 private:
  void recursiveKnapsack(const std::unordered_map<int, int>::iterator& candidatesIterator,
                         std::vector<std::vector<int>>& answers, std::vector<int>& added, const int& target,
                         const std::unordered_map<int, int>::iterator& candidatesEnd) {
    if (target < 0) return;
    if (target == 0) {
      answers.push_back(added);
      return;
    }
    if (candidatesIterator == candidatesEnd) return;

    for (int i = 0; i <= candidatesIterator->second; ++i) {
      for (int j = 0; j < i; ++j) added.emplace_back(candidatesIterator->first);
      recursiveKnapsack(std::next(candidatesIterator), answers, added, target - i * candidatesIterator->first,
                        candidatesEnd);
      for (int j = 0; j < i; ++j) added.pop_back();
    }
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<std::vector<int>, int>> cases{{{10, 1, 2, 7, 6, 1, 5}, 8}, {{2, 5, 2, 1, 2}, 5}};
  for (auto& [vector, target] : cases) {
    const auto& answer = s.combinationSum2(vector, target);
    // print answer
    std::cout << "Answer:\n";
    for (const auto& combination : answer) {
      std::copy(combination.begin(), combination.end(), std::ostream_iterator<int>(std::cout, " "));
      std::cout << '\n';
    }
  }
}
