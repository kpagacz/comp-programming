// link to the problem: https://leetcode.com/problems/string-to-integer-atoi/
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
    int myAtoi(string s) {
        auto l = std::atol(s.c_str());
      if (l > INT_MAX) return INT_MAX;
      if (l < INT_MIN) return INT_MIN;
      return (int) l;
    }
};

int main(int argc, char** argv) {}
