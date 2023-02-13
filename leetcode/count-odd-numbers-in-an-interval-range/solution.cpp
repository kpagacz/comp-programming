// link to the problem: https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/description/
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
    int countOdds(int low, int high) {
      if ((high - low + 1) % 2 == 0) return (high - low + 1) / 2;
      else return (high - low + 1 + (low % 2)) / 2;
    }
};

int main(int argc, char** argv) {}
