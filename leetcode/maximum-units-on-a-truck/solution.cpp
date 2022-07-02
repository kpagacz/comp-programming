// link to the problem: https://leetcode.com/problems/maximum-units-on-a-truck/
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

// I could make it less stupid - the inner loop is not needed.
// Runtime: 53 ms, faster than 77.51% of C++ online submissions for Maximum Units on a Truck.
// Memory Usage: 16 MB, less than 53.55% of C++ online submissions for Maximum Units on a Truck.

class Solution {
 public:
  int maximumUnits(std::vector<std::vector<int>>& boxTypes, int truckSize) {
    std::sort(boxTypes.begin(), boxTypes.end(),
              [](const auto& first, const auto& second) { return first[1] > second[1]; });
    int totalUnits = 0;
    auto it = boxTypes.begin();
    while (truckSize && it != boxTypes.end()) {
      int newBoxes = std::min(truckSize, (*it)[0]);
      truckSize -= newBoxes;
      totalUnits += newBoxes * (*it)[1];
      it++;
    }
    return totalUnits;
  }
};

int main(int argc, char** argv) {}
