// link to the problem:
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Orderly Queue.
// Memory Usage: 6.3 MB, less than 91.95% of C++ online submissions for Orderly Queue.

class Solution {
 public:
  std::string orderlyQueue(std::string s, int k) {
    if (k > 1) {
      std::sort(s.begin(), s.end());
      return s;
    }
    std::string lowest(s);

    for (int i = 0; i < s.size(); ++i) {
      std::rotate(s.begin(), s.begin() + 1, s.end());
      lowest = std::min(lowest, s);
    }

    return lowest;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << s.orderlyQueue("cba", 1);
  std::cout << '\n';
}
