// link to the problem: https://leetcode.com/problems/car-pooling/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <vector>
#include<unordered_map>

class Solution {
 public:
  bool carPooling(std::vector<std::vector<int>>& trips, int capacity) {
    std::vector<int> coords;
    std::for_each(trips.begin(), trips.end(), [&](const auto& d) {
      coords.push_back(d[1]);
      coords.push_back(d[2]);
    });
    std::sort(coords.begin(), coords.end());
    coords.erase(std::unique(coords.begin(), coords.end()), coords.end());

    std::copy(coords.begin(), coords.end(), std::ostream_iterator<int>(std::cout, " "));

    std::unordered_map<int, int> pos;
    for (auto i{0}; i < coords.size(); i++) pos.insert({coords[i], i});
    std::vector<int> passengers(coords.size(), 0);

    std::for_each(trips.begin(), trips.end(), [&](const auto& trip) {
      auto first_stop = pos.at(trip[1]);
      auto second_stop = pos.at(trip[2]);
      for (auto j{first_stop}; j < second_stop; j++) passengers[j] += trip[0];
    });

    auto predicate = [&](const auto& n) { return n <= capacity; };
    return std::all_of(passengers.begin(), passengers.end(), predicate);
  }
};

int main(int argc, char** argv) {}
