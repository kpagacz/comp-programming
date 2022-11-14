// link to the problem: https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
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

// Runtime: 77 ms, faster than 89.01% of C++ online submissions for Most Stones Removed with Same Row or Column.
// Memory Usage: 17.3 MB, less than 47.13% of C++ online submissions for Most Stones Removed with Same Row or Column.

class Solution {
 public:
  int removeStones(std::vector<std::vector<int>>& stones) {
    for (const auto& stone : stones) {
      int stoneNo = parents.size();
      addSet();
      // std::cout << "iteration: " << stoneNo << '\n';
      // std::cout << "parents: ";
      // std::copy(parents.begin(), parents.end(), std::ostream_iterator<int>(std::cout, " "));
      // std::cout << '\n';
      if (rowStones.count(stone[0]) > 0 && colStones.count(stone[1]) > 0) {
        int rowRoot = root(rowStones[stone[0]]), colRoot = root(colStones[stone[1]]);
        mergeSets(rowRoot, colRoot);
        parents[stoneNo] = root(rowRoot);
      } else if (rowStones.count(stone[0])) {
        int rowRoot = root(rowStones[stone[0]]);
        parents[stoneNo] = rowRoot;
        colStones[stone[1]] = stoneNo;
      } else if (colStones.count(stone[1])) {
        int colRoot = root(colStones[stone[1]]);
        parents[stoneNo] = colRoot;
        rowStones[stone[0]] = stoneNo;
      } else {
        rowStones[stone[0]] = stoneNo;
        colStones[stone[1]] = stoneNo;
        parents[stoneNo] = stoneNo;
      }
      // std::cout << "parents: ";
      // std::copy(parents.begin(), parents.end(), std::ostream_iterator<int>(std::cout, " "));
      // std::cout << '\n';
    }

    return parents.size() - countSets();
  }

 private:
  std::vector<int> parents, ranks;
  std::unordered_map<int, int> rowStones, colStones;

  void addSet() {
    parents.push_back(parents.size());
    ranks.push_back(0);
  }

  void mergeSets(const int& first, const int& second) {
    if (root(first) == root(second)) return;
    int firstRoot = root(first), secondRoot = root(second);
    assert(parents[firstRoot] == firstRoot);
    assert(parents[secondRoot] == secondRoot);
    if (ranks[firstRoot] == ranks[secondRoot]) {
      parents[secondRoot] = firstRoot;
      ranks[firstRoot]++;
      return;
    }
    if (ranks[firstRoot] < ranks[secondRoot]) std::swap(firstRoot, secondRoot);
    parents[secondRoot] = firstRoot;
  }

  int root(const int& set) {
    // with path compression
    if (parents[set] == set) return set;
    int rootSet = root(parents[set]);
    parents[set] = rootSet;
    return rootSet;
  }

  int countSets() {
    int sets = 0;
    for (int i = 0; i < parents.size(); ++i)
      if (parents[i] == i) sets++;
    return sets;
  }
};

int main(int argc, char** argv) {
  std::vector<std::vector<std::vector<int>>> cases;
  cases.push_back({{0, 0}, {0, 1}});
  cases.push_back({{0, 0}, {0, 1}, {1, 0}, {1, 2}, {2, 1}, {2, 2}});
  for (auto stones : cases) {
    Solution s;
    std::cout << s.removeStones(stones) << '\n';
  }
}
