// link to the problem: https://leetcode.com/problems/generate-parentheses/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <vector>

class Solution {
 public:
  std::vector<std::string> generateParenthesis(int n) {
    auto is_valid = [](const auto& permutation) {
      std::stack<int> pars;
      for (const auto& d : permutation) {
        switch (d) {
          case 0:
            pars.push(d);
            break;
          case 1:
            if (pars.empty()) return false;
            else pars.pop();
            break;
        }
      }
      return pars.empty();
    };
    std::vector<std::string> ans;

    std::vector<int> pars(2 * n);
    std::fill_n(pars.begin(), n, 0);
    std::fill_n(pars.begin() + n, n, 1);
    std::sort(pars.begin(), pars.end());
    do {
      if (!is_valid(pars)) continue;
      std::string el(2 * n, ' ');
      std::transform(pars.begin(), pars.end(), el.begin(), [](const auto& c) { return c ? ')' : '('; });
      ans.push_back(el);
    } while (std::next_permutation(pars.begin(), pars.end()));

    return ans;
  }
};

int main(int argc, char** argv) {
  Solution s;
  for (const auto& s : s.generateParenthesis(4)) {
    std::cout << s << " ";
  }
}
