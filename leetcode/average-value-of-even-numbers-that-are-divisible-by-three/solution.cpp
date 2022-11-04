// link to the problem: https://leetcode.com/problems/average-value-of-even-numbers-that-are-divisible-by-three/
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

// Runtime: 10 ms, faster than 91.02% of C++ online submissions for Average Value of Even Numbers That Are Divisible by
// Three. Memory Usage: 13.6 MB, less than 60.33% of C++ online submissions for Average Value of Even Numbers That Are
// Divisible by Three.

class Solution {
 public:
  int averageValue(std::vector<int>& nums) {
    int sum = 0, count = 0;
    auto sumEvenDivisibleByThree = [&](const int& num) {
      if (num % 6 == 0) {
        count++;
        sum += num;
      }
    };

    std::for_each(nums.begin(), nums.end(), sumEvenDivisibleByThree);
    return count == 0 ? 0 : sum / count;
  }
};

int main(int argc, char** argv) {}
