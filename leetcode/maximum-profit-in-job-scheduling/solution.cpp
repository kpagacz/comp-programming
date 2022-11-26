// link to the problem: https://leetcode.com/problems/maximum-profit-in-job-scheduling/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <map>
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

// Runtime: 272 ms, faster than 68.34% of C++ online submissions for Maximum Profit in Job Scheduling.
// Memory Usage: 55.4 MB, less than 65.64% of C++ online submissions for Maximum Profit in Job Scheduling.

class Solution {
 public:
  int jobScheduling(std::vector<int>& startTime, std::vector<int>& endTime, std::vector<int>& profit) {
    std::vector<int> jobsSortedByTime(startTime.size());
    std::iota(jobsSortedByTime.begin(), jobsSortedByTime.end(), 0);
    auto sortByTime = [&](const int& first, const int& second) {
      return endTime[first] < endTime[second]    ? true
             : endTime[first] == endTime[second] ? startTime[first] <= startTime[second]
                                                 : false;
    };
    std::sort(jobsSortedByTime.begin(), jobsSortedByTime.end(), sortByTime);

    // std::cout << "Sorted jobs:\n";
    // for (const auto& job : jobsSortedByTime)
    //   std::cout << "Start time: " << startTime[job] << " end time: " << endTime[job] << " profit: " << profit[job]
    //             << '\n';

    std::map<int, int> dp;  // key: time; value: maximum profit at that time
    dp.insert({0, 0});
    int maxSoFar = 0;
    for (const auto& job : jobsSortedByTime) {
      auto maxProfitUntilJobStart = std::next(dp.upper_bound(startTime[job]), -1)->second;
      // std::cout << "Max profit so far: " << maxProfitUntilJobStart << '\n';
      // std::cout << "Max ever: " << maxSoFar << '\n';
      if (dp.count(endTime[job]) == 0) dp[endTime[job]] = std::max(maxSoFar, maxProfitUntilJobStart + profit[job]);
      else {
        dp[endTime[job]] = std::max(dp[endTime[job]], maxProfitUntilJobStart + profit[job]);
        dp[endTime[job]] = std::max(dp[endTime[job]], maxSoFar);
      }
      maxSoFar = dp[endTime[job]];
    }

    return maxSoFar;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::tuple<std::vector<int>, std::vector<int>, std::vector<int>>> cases;
  cases.push_back({{1, 2, 3, 3}, {3, 4, 5, 6}, {50, 10, 40, 70}});
  cases.push_back({{1, 2, 3, 1}, {3, 4, 5, 6}, {50, 10, 40, 70}});
  cases.push_back({{1, 2, 3, 4, 6}, {3, 5, 10, 6, 9}, {20, 20, 100, 70, 60}});
  cases.push_back({{1, 1, 1}, {2, 3, 4}, {5, 6, 4}});
  cases.push_back({{4, 2, 4, 8, 2}, {5, 5, 5, 10, 8}, {1, 2, 8, 10, 4}});
  for (auto [starTime, endTime, profit] : cases) std::cout << s.jobScheduling(starTime, endTime, profit) << '\n';
}
