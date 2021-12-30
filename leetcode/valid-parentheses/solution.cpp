// link to the problem: https://leetcode.com/problems/valid-parentheses/
#include<iostream>
#include<vector>
#include<array>
#include<string>
#include<algorithm>
#include<numeric>
#include<sstream>
#include<iterator>
#include<stack>

class Solution {
public:
    bool isValid(std::string s) {
      std::stack<char> stack;
      for(const auto& c : s) {
        switch(c) {
          case '(':
          case '{':
          case '[':
            stack.push(c);
            break;
          case ')':
            if (stack.empty() || stack.top() != '(') return false;
            stack.pop();
            break;
          case ']':
            if (stack.empty() || stack.top() != '[') return false;
            stack.pop();
            break;
          case '}':
            if (stack.empty() || stack.top() != '{') return false;
            stack.pop();
            break;
        }
      }
      return stack.empty();
    }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << std::boolalpha << s.isValid("()");
}
