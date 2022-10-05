// link to the problem: https://leetcode.com/problems/number-of-common-factors/
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

constexpr int MAX = 1000;

class Solution {
 public:
  int commonFactors(int a, int b) {
    int answer = 0;
    for (int i = 1; i <= a && i <= b && i <= MAX; ++i)
      if (a % i == 0 && b % i == 0) ++answer;
    return answer;
  }
};

int main(int argc, char** argv) {}
