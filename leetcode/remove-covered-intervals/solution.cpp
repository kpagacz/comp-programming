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

class Solution {
 public:
  int removeCoveredIntervals(std::vector<std::vector<int>>& intervals) {
    int covered = 0;
    auto sorter = [](const std::vector<int>& a, const std::vector<int>& b) { return a[0] == b[0] ? a[1] > b[1] : a[0] < b[0]; };
    std::sort(intervals.begin(), intervals.end(), sorter);
    // for (const auto& pair : intervals) std::cout << pair[0] << " " << pair[1] << '\n';

    const std::vector<int>* active = nullptr;
    for(const auto& interval : intervals) {
      if (active == nullptr || interval[1] > (*active)[1]) {
        active = &interval;
      } else {
        covered++;
      }
    }

    return intervals.size() - covered;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::vector<int>> intervals{{1, 4}, {3, 6}, {2, 8}, {3,9}};
  std::cout << s.removeCoveredIntervals(intervals);
}
