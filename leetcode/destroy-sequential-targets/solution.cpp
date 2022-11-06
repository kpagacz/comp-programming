// link to the problem: https://leetcode.com/problems/destroy-sequential-targets/
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

// Runtime: 365 ms, faster than 59.04% of C++ online submissions for Destroy Sequential Targets.
// Memory Usage: 58.9 MB, less than 92.61% of C++ online submissions for Destroy Sequential Targets.

class Solution {
 public:
  int destroyTargets(std::vector<int>& nums, int space) {
    std::unordered_map<int, int> modulos;
    for (const auto& num : nums) modulos[num % space]++;
    auto maxElementsModulo = INT32_MIN;
    for (const auto& [modulo, elements] : modulos) maxElementsModulo = std::max(maxElementsModulo, elements);
    std::unordered_set<int> mostElementsModulos;
    for (const auto& [modulo, elements] : modulos)
      if (elements == maxElementsModulo) mostElementsModulos.insert(modulo);

    int minElement = INT32_MAX;
    for (const auto& num : nums)
      if (mostElementsModulos.count(num % space)) minElement = std::min(minElement, num);
    return minElement;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<std::vector<int>, int>> cases;
  cases.push_back({{2, 3, 4, 1, 4, 5, 3, 4, 5, 2}, 10000});
  for (auto [nums, space] : cases) std::cout << s.destroyTargets(nums, space) << '\n';
}
