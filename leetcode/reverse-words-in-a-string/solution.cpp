// link to the problem: https://leetcode.com/problems/reverse-words-in-a-string/
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

// Runtime: 4 ms, faster than 91.32% of C++ online submissions for Reverse Words in a String.
// Memory Usage: 7.8 MB, less than 56.97% of C++ online submissions for Reverse Words in a String.

class Solution {
 public:
  std::string reverseWords(std::string s) {
    std::stringstream ss(s);
    std::string word;
    std::stack<std::string> words;
    while (ss >> word) {
      words.push(word);
    }
    std::string answer;
    while (!words.empty()) answer += words.top() + " ", words.pop();
    answer.pop_back();
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::string> cases;
  cases.push_back(" hello world  ");
  cases.push_back("a good   example");
  cases.push_back("the sky is blue");
  for (auto sentence : cases) std::cout << s.reverseWords(sentence) << '\n';
}
