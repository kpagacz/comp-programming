// link to the problem: https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Minimum Recolors to Get K Consecutive Black Blocks.
// Memory Usage: 6.2 MB, less than 11.11% of C++ online submissions for Minimum Recolors to Get K Consecutive Black
// Blocks.

class Solution {
 public:
  int minimumRecolors(std::string blocks, int k) {
    int window = 0;
    for (int i = 0; i < k; i++)
      if (blocks[i] == 'W') window++;
    int answer = window;
    for (int i = k; i < blocks.size(); ++i) {
      if (blocks[i - k] == 'W') window--;
      if (blocks[i] == 'W') window++;
      answer = std::min(answer, window);
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
