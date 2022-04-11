// link to the problem: https://leetcode.com/problems/implement-strstr/
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
 private:
  std::vector<int> kmp_preprocess(std::string pattern) {
    std::vector<int> answer(pattern.size() + 1, 0);
    int position = 1;   // the currently computed position in answer
    int candidate = 0;  // the zero-based index in the pattern of the next character of the current candidate prefix
    answer[0] = -1;
    while (position < pattern.size()) {
      if (pattern[position] == pattern[candidate]) {
        answer[position] = answer[candidate];
      } else {
        answer[position] = candidate;
        while (candidate >= 0 && pattern[position] != pattern[candidate]) candidate = answer[candidate];
      }
      position++;
      candidate++;
    }
    answer[position] = candidate;
    return answer;
  }

 public:
  int strStr(std::string haystack, std::string needle) {
    if (needle == "") return 0;
    auto kmp = kmp_preprocess(needle);
    int answer = -1;
    int m = 0, i = 0;
    while(m < haystack.size()) {
      while(i < needle.size()) {
        if (haystack[m + i] != needle[i]) break;
        i++;
      }
      if (i == needle.size()) return m;
      m += i - kmp[i];
      i = std::max(0, kmp[i]);
    }
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  s.strStr("haystack", "aaa");
}
