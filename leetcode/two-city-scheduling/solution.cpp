// link to the problem: https://leetcode.com/problems/two-city-scheduling/
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
  int twoCitySchedCost(std::vector<std::vector<int>>& costs) {
    std::sort(costs.begin(), costs.end(),
              [](const auto& a, const auto& b) { std::abs(a[0] - a[1]) > std::abs(b[0], b[1]); });
    for(const auto& cost : costs) {
      std::cout << cost[0] << " " << cost[1] << '\n';
    }

    int answer = 0, first_city = 0, second_city = 0;
    for(const auto& cost : costs) {
      if (cost[0] < cost[1]) {
        if (first_city < costs.size() / 2) {
          answer += cost[0];
          first_city++;
        } else {
          answer += cost[1];
          second_city++;
        }
      } else {
        if (second_city < costs.size() / 2) {
          answer += cost[1];
          second_city++;
        } else {
          answer += cost[0];
          first_city++;
        }
      }
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
