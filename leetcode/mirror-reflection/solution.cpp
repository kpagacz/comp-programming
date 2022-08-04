// link to the problem: https://leetcode.com/problems/mirror-reflection/
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
  int mirrorReflection(int p, int q) {
    while (p % 2 == 0 && q % 2 == 0) {
      p /= 2;
      q /= 2;
    }

    if (p % 2) return 0;
    if (p % 2 && q % 2) return 1;
    return 2;
  }
};

int main(int argc, char** argv) {}
