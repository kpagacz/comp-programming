// link to the problem: https://leetcode.com/problems/maximum-segment-sum-after-removals/
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

// What did I learn?
// You can implement union find super fast using an array with minimal code. That is wow.
// Also, a nice trick to sneak in values of the sets is to set them as negative in the union find array.
// If the set values are always of one sign it works..., but it does not have to be the case.
// If it is not, you need to keep another array to store the values or use an array of nodes instead of integers.

// Runtime: 317 ms, faster than 100.00% of C++ online submissions for Maximum Segment Sum After Removals.
// Memory Usage: 102.1 MB, less than 100.00% of C++ online submissions for Maximum Segment Sum After Removals.

class Solution {
 public:
  std::vector<long long> maximumSegmentSum(std::vector<int>& nums, std::vector<int>& removeQueries) {
    std::vector<long long> unionFind(nums.size(), INT_MAX), answer(removeQueries.size());
    for (int i = removeQueries.size() - 1; i > 0; i--) {
      int indexToAdd = removeQueries[i];
      unionFind[indexToAdd] = -nums[indexToAdd];
      if (indexToAdd > 0 && unionFind[indexToAdd - 1] != INT_MAX) merge(indexToAdd, indexToAdd - 1, unionFind);
      if (indexToAdd < nums.size() - 1 && unionFind[indexToAdd + 1] != INT_MAX)
        merge(indexToAdd, indexToAdd + 1, unionFind);
      answer[i - 1] = std::max(answer[i], -unionFind[find(indexToAdd, unionFind)]);
    }
    return answer;
  }

  int find(const int& i, std::vector<long long>& unionFind) {
    return unionFind[i] < 0 ? i : find(unionFind[i], unionFind);
  }

  void merge(const int& firstSet, const int& secondSet, std::vector<long long>& unionFind) {
    int firstRoot = find(firstSet, unionFind), secondRoot = find(secondSet, unionFind);
    unionFind[secondRoot] += unionFind[firstRoot];
    unionFind[firstRoot] = secondRoot;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<std::vector<int>, std::vector<int>>> tests;
  tests.push_back({{1, 2, 5, 6, 1}, {0, 3, 2, 4, 1}});
  for (auto& [nums, removeQueries] : tests) {
    auto answer = s.maximumSegmentSum(nums, removeQueries);
    std::copy(answer.begin(), answer.end(), std::ostream_iterator<long long>(std::cout, " "));
    std::cout << '\n';
  }
}
