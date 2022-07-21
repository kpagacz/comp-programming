// link to the problem: https://leetcode.com/problems/happy-number/
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
  bool isHappy(int n) {
    std::unordered_set<int> seen;
    while (n != 1 && seen.count(n) == 0) {
      seen.insert(n);
      int temp = 0;
      while (n > 0) {
        temp += (n % 10) * (n % 10);
        n /= 10;
      }
      n = temp;
    }
    return n == 1;
  }
};

int main(int argc, char** argv) {}
