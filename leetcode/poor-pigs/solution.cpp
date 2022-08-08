// link to the problem: https://leetcode.com/problems/poor-pigs/
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
  int poorPigs(int buckets, int minutesToDie, int minutesToTest) {
    return std::ceil(std::log2(buckets) / std::log2(minutesToTest / minutesToDie + 1));
  }
};

int main(int argc, char** argv) {}
