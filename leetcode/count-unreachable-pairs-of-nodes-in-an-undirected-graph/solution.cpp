// link to the problem: https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
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
  long long countPairs(int n, std::vector<std::vector<int>>& edges) {
    std::vector<std::pair<int, int64_t>> unionSet(n, {-1, 1});
    for (const auto& edge : edges) merge(unionSet, edge[0], edge[1]);

    return std::transform_reduce(unionSet.begin(), unionSet.end(), (int64_t)0, std::plus<int64_t>(),
                                 [&](const auto& set) { return set.first == -1 ? set.second * (n - set.second) : 0; }) /
           2;
  };

  int find(std::vector<std::pair<int, int64_t>>& unionSet, const int& node) {
    if (unionSet[node].first == -1) return node;
    unionSet[node].first = find(unionSet, unionSet[node].first);
    return unionSet[node].first;
  }

  void merge(std::vector<std::pair<int, int64_t>>& unionSet, const int& first, const int& second) {
    auto firstRoot = find(unionSet, first);
    auto secondRoot = find(unionSet, second);

    if (firstRoot == secondRoot) return;
    unionSet[firstRoot].first = secondRoot;
    unionSet[secondRoot].second += unionSet[firstRoot].second;
  }
};

int main(int argc, char** argv) {}
