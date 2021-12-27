// link to the problem https://leetcode.com/problems/number-complement/
#include<iostream>

class Solution {
 public:
  int findComplement(int num) {
    int bits = log2(num) + 1;
    for (int i = 0; i < bits; i++) {
      num = num ^ (1 << i);
    }

    return num;
  }
};
