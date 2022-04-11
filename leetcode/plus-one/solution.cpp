// link to the problem: https://leetcode.com/problems/plus-one/
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
  std::vector<int> plusOne(std::vector<int>& digits) {
    int it = digits.size() - 1;
    digits[it]++;

    while(it > 0 && digits[it] >= 10) {
      digits[it - 1] += digits[it] / 10;
      digits[it] -= 10;
      it--;
    }
    if (digits[0] >= 10) {
      digits.insert(digits.begin(), digits[0] / 10);
      digits[1] - 10;
    }

    return digits;
  }
};

int main(int argc, char** argv) {}
