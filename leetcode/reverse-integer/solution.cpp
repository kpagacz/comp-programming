// link to the problem: https://leetcode.com/problems/reverse-integer/
#include<iostream>
#include<vector>
#include<array>
#include<string>
#include<algorithm>
#include<numeric>
#include<sstream>
#include<iterator>
#include<queue>
#include <stdexcept>

class Solution {
public:
    int reverse(int x) {
      std::string number = std::to_string(x);
      if (number[0] == '-') {
        std::reverse(number.begin() + 1, number.end());
      } else {
        std::reverse(number.begin(), number.end());
      }

      try {
        return std::stoi(number, 0, 10);
      } catch (std::out_of_range exc) {
        return 0;
      }
    }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << s.reverse(2147483647);
}
