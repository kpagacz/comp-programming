// link to the problem: https://leetcode.com/problems/word-pattern/
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
#include <unordered_map>
#include <vector>

class Solution {
 public:
  bool wordPattern(std::string pattern, std::string s) {
    std::vector<std::vector<int>> positions(26);
    for (int i = 0; i < pattern.size(); i++) {
      positions[pattern[i] - 'a'].push_back(i);
    }

    std::istringstream ss(s);
    std::vector<std::string> words((std::istream_iterator<std::string>(ss)), (std::istream_iterator<std::string>()));

    int positions_sum = 0;
    for (const auto& v : positions) positions_sum += v.size();
    if (positions_sum != words.size()) return false;
    for (const auto& v : positions) {
      if (!v.empty()) {
        const auto first = v[0];
        for (const auto& p : v) {
          if (words[p] != words[first]) return false;
        }
        for (int i = 0; i < words.size(); i++) {
          if ((std::find(v.begin(), v.end(), i) == v.end()) && words[first] == words[i]) return false;
        }
      }
    }
    return true;
  }
};

int main(int argc, char** argv) {
  std::string pattern = "ab";
  std::string words = "cat cat";
  Solution s;
  std::cout << std::boolalpha << s.wordPattern(pattern, words);
}
