// link to the problem: https://leetcode.com/problems/longest-valid-parentheses/
// What have I learned:
// * this problem can be solved in another way: linear scan from the beginning and then linear scan from the end to the
// beginning. Keeping the track of the number of left and right brackets, when left == right, it means the currently
// considered substring is valid and its length should be compared to the max. When the number of "closing" brackets
// exceeds the number of opening brackets, the index of the currently considered substring is reset to the current
// character
// * my stack solution can also be simplified, I don't have to use the opening parentheses, simply pushing to the stack
// on every non-fitting ')' and checking if the stack is empty does the trick
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
  int longestValidParentheses(std::string s) {
    int current = 0, longest = 0;
    int opening_parentheses = 0;
    std::stack<int> maxes;
    for (const auto& c : s) {
      switch (c) {
        case '(':
          opening_parentheses++;
          maxes.push(current);
          current = 0;
          break;
        case ')':
          if (opening_parentheses > 0) {
            opening_parentheses--;
            current += maxes.top();
            maxes.pop();
            current += 2;
          } else {
            maxes.push(current);
            current = 0;
          }
          break;
      }
    }
    maxes.push(current);
    while (!maxes.empty()) {
      longest = std::max(longest, maxes.top());
      maxes.pop();
    }
    std::cout << '\n';
    return longest;
  }
};
int main(int argc, char** argv) {}
