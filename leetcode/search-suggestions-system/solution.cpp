// link to the problem: https://leetcode.com/problems/search-suggestions-system/
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
  std::vector<std::vector<std::string>> suggestedProducts(std::vector<std::string>& products, std::string searchWord) {
    std::sort(products.begin(), products.end());
    std::vector<std::vector<std::string>> answer;
    for (auto length{1}; length <= searchWord.size(); ++length) {
      std::string_view prefix(searchWord.data(), length);
      std::vector<std::string> suggestions;
      auto lower_bound = std::lower_bound(products.begin(), products.end(), prefix);
      // std::cout << "prefix: " << prefix << '\n';
      for (auto i{0}; i < 3 && lower_bound != products.end(); i++) {
        // std::cout << "it: " << i << " ";
        // std::cout << std::boolalpha << ((*lower_bound).substr(0, prefix.size()).compare(prefix) == 0) << '\n';
        if ((*lower_bound).substr(0, prefix.size()).compare(prefix) == 0) suggestions.push_back(*lower_bound);
        std::advance(lower_bound, 1);
      }
      answer.push_back(suggestions);
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
