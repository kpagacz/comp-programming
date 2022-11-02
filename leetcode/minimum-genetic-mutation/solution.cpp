// link to the problem: https://leetcode.com/problems/minimum-genetic-mutation/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Minimum Genetic Mutation.
// Memory Usage: 6.6 MB, less than 42.79% of C++ online submissions for Minimum Genetic Mutation.

constexpr std::array<char, 4> bases{'A', 'G', 'C', 'T'};

class Solution {
 public:
  int minMutation(std::string start, std::string end, std::vector<std::string>& bank) {
    std::queue<std::tuple<int, std::string>> mutations;  // distance, currentString
    std::unordered_map<std::string, bool> visited;
    for (const auto& gene : bank) visited.insert({gene, false});
    mutations.push({0, start});

    while (!mutations.empty()) {
      auto [distance, gene] = mutations.front();
      mutations.pop();
      if (gene == end) return distance;
      visited[gene] = true;

      for (int i = 0; i < gene.size(); ++i) {
        char originalBase = gene[i];
        for (const auto& base : bases) {
          gene[i] = base;
          if (visited.count(gene) > 0 && visited[gene] == false && gene != start) mutations.push({distance + 1, gene});
        }
        gene[i] = originalBase;
      }
    }

    return -1;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::tuple<std::string, std::string, std::vector<std::string>>> cases;

  cases.push_back({"AACCGGTT", "AACCGGTA", {"AACCGGTA"}});
  cases.push_back({"AACCGGTT", "AAACGGTA", {"AACCGGTA", "AACCGCTA", "AAACGGTA"}});
  cases.push_back({"AAAAACCC", "AACCCCCC", {"AAAACCCC", "AAACCCCC", "AACCCCCC"}});
  for (auto [start, end, bank] : cases) {
    std::cout << s.minMutation(start, end, bank) << '\n';
  }
}
