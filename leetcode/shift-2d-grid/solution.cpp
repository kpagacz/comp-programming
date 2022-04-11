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
    std::vector<std::vector<int>> shiftGrid(std::vector<std::vector<int>>& grid, int k) {
      std::vector<int> oneD;
      for(const auto& v : grid)
        for (const auto& el : v) oneD.push_back(el);
      std::rotate(oneD.rbegin(), oneD.rbegin() + (k % oneD.size()), oneD.rend());
      int it = 0;
      for (auto& v : grid)
        for (auto& el : v) el = oneD[it++];
      return grid;
    }
};

int main(int argc, char** argv) {

}
