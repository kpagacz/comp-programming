// link to the problem: https://leetcode.com/problems/reverse-bits/
#include <algorithm>
#include <array>
#include <bitset>
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
  uint32_t reverseBits(uint32_t n) {
    std::bitset<32> bits(n);
    auto stringBits = bits.to_string();
    std::reverse(stringBits.begin(), stringBits.end());
    return std::bitset<32>(stringBits).to_ulong();
  }
};

int main(int argc, char** argv) {}
