// link to the problem: https://leetcode.com/problems/minimize-xor/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Minimize XOR.
// Memory Usage: 6 MB, less than 32.51% of C++ online submissions for Minimize XOR.

class Solution {
 public:
  int minimizeXor(int num1, int num2) {
    std::bitset<32> answer(0);
    std::bitset<32> num1bits(num1);
    int setBits = std::bitset<32>(num2).count();
    for (int i = 31; i >= 0 && setBits > 0; --i)
      if (num1bits[i]) {
        answer[i].flip();
        --setBits;
      }
    for (int i = 0; i < 32 && setBits > 0; ++i)
      if (num1bits[i] == false) {
        --setBits;
        answer[i].flip();
      }
    return answer.to_ulong();
  }
};

int main(int argc, char** argv) {}
