// link to the problem: https://leetcode.com/problems/number-of-1-bits/
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
  int hammingWeight(uint32_t n) {
    int answer = 0;
    while (n) answer += n & 1, n >>= 1;
    return answer;
  }
};
int main(int argc, char** argv) {}
