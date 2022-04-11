// link to the problem: https://leetcode.com/problems/add-digits/
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
  int addDigits(int num) {
    int sum = 0;
    while(num > 0) {
      sum += num % 10;
      num /= 10;
    }
    if (sum < 10)
      return sum;
    else
      return addDigits(sum);
  }
};

int main(int argc, char** argv) {}
