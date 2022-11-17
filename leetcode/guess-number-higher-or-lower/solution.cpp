// link to the problem: https://leetcode.com/problems/guess-number-higher-or-lower/
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

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * int guess(int num);
 */

class Solution {
 public:
  int guessNumber(int n) {
    int left = 1, right = n;
    while (left <= right) {
      int middle = left / 2 + right / 2 + (left & right & 1);
      auto compared = guess(middle);
      if (compared == 0) return middle;
      else if (compared > 0) {  // middle is lower than the guessed number
        left = middle + 1;
      } else {  // middle is higher than the guessed number
        right = middle;
      }
    }
    return -1;
  }
};

int main(int argc, char** argv) {
  int max = (1 << 31) - 1;
}
