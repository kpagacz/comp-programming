// link to the problem: https://leetcode.com/problems/broken-calculator/
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
  int brokenCalc(int startValue, int target) {
    int answer = 0;
    while (target > startValue) {
      if (target % 2 == 1)
        target += 1;
      else
        target /= 2;
      answer++;
    }
    return answer + startValue - target;
  }
};

int main(int argc, char** argv) {}
