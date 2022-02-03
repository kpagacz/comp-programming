// link to the problem: https://leetcode.com/problems/find-all-anagrams-in-a-string/
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

void printVector(std::vector<int> v) {
  std::copy(v.begin(), v.end(), std::ostream_iterator<int>(std::cout, " "));
  std::cout << '\n';
}

class Solution {
public:
    bool vectors_equal(std::vector<int> a, std::vector<int> b) {
      bool equal = true;
      for (auto i{0}; i < a.size(); i++)
        equal = equal && a[i] == b[i];
      return equal;
    }

    std::vector<int> findAnagrams(std::string s, std::string p) {
      std::vector<int> answer;
      std::vector<int> pattern_letters(26, 0);
      for(const auto& c : p)
        pattern_letters[c - 'a']++;

      std::vector<int> sliding_window(26, 0);

      int begin = 0, end = std::accumulate(pattern_letters.begin(), pattern_letters.end(), 0) - 1;
      if (end >= s.size())
        return answer;


      for (auto i{begin}; i <= end; i++)
        sliding_window[s[i] - 'a']++;
      int target = 26;
      int current = 0;
      for (auto i{0}; i < pattern_letters.size(); i++)
        if (sliding_window[i] == pattern_letters[i])
          current++;

      while (end < s.size() - 1){
        if (vectors_equal(sliding_window, pattern_letters))
          answer.push_back(begin);

        sliding_window[s[begin++] - 'a']--;
        sliding_window[s[++end] - 'a']++;
      }
      if (vectors_equal(sliding_window, pattern_letters))
          answer.push_back(begin);

      return answer;
    }
};

int main(int argc, char** argv) {
  Solution s;
  auto answer = s.findAnagrams("cbaebabacd", "abc");
  printVector(answer);
}
