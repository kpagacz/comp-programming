// link to the problem: https://leetcode.com/problems/maximum-segment-sum-after-removals/
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

// Runtime: 509 ms, faster than 66.67% of C++ online submissions for Maximum Segment Sum After Removals.
// Memory Usage: 131.3 MB, less than 16.67% of C++ online submissions for Maximum Segment Sum After Removals.

class Solution {
 public:
  std::vector<long long> maximumSegmentSum(std::vector<int>& nums, std::vector<int>& removeQueries) {
    // std::priority_queue<long long> segmentSums;
    std::vector<std::tuple<int, int, long long>> segments;  // start, end, sum
    segments.reserve(nums.size());
    std::unordered_map<int, std::tuple<int, int, long long>*> starts, ends;
    std::vector<long long> answer{0};
    long long currentMax = 0;

    for (auto it = removeQueries.rbegin(); it != removeQueries.rend() - 1; it++) {
      const auto& indexToAdd = *it;
      // std::cout << "Index to add: " << indexToAdd << " value: " << nums[indexToAdd] << '\n';
      if (starts.count(indexToAdd + 1) && ends.count(indexToAdd - 1)) {
        auto nextSegment = starts[indexToAdd + 1];
        auto previousSegment = ends[indexToAdd - 1];
        segments.push_back({std::get<0>(*previousSegment), std::get<1>(*nextSegment),
                            std::get<2>(*previousSegment) + std::get<2>(*nextSegment) + nums[indexToAdd]});
        ends.erase(indexToAdd - 1);
        starts.erase(indexToAdd + 1);
        // segmentSums.push(std::get<2>(segments.back()));
        currentMax = std::max(currentMax, std::get<2>(segments.back()));
        starts[std::get<0>(*previousSegment)] = &(segments.back());
        ends[std::get<1>(*nextSegment)] = &(segments.back());
      } else if (starts.count(indexToAdd + 1)) {
        auto segment = starts[indexToAdd + 1];
        std::get<0>(*segment) = indexToAdd;
        std::get<2>(*segment) += nums[indexToAdd];
        // segmentSums.push(std::get<2>(*segment));
        currentMax = std::max(currentMax, std::get<2>(*segment));
        starts.erase(indexToAdd + 1);
        starts[indexToAdd] = segment;
      } else if (ends.count(indexToAdd - 1)) {
        auto segment = ends[indexToAdd - 1];
        std::get<1>(*segment) = indexToAdd;
        std::get<2>(*segment) = std::get<2>(*segment) + nums[indexToAdd];
        ends.erase(indexToAdd - 1);
        ends[indexToAdd] = segment;
        // segmentSums.push(std::get<2>(*segment));
        currentMax = std::max(currentMax, std::get<2>(*segment));
      } else {
        // segmentSums.push(nums[indexToAdd]);
        currentMax = std::max(currentMax, 0LL + nums[indexToAdd]);
        segments.push_back({indexToAdd, indexToAdd, nums[indexToAdd]});
        starts[indexToAdd] = &(segments.back());
        ends[indexToAdd] = &(segments.back());
      }
      // answer.push_back(segmentSums.top());
      answer.push_back(currentMax);
    }

    std::reverse(answer.begin(), answer.end());
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<std::vector<int>, std::vector<int>>> tests;
  tests.push_back({{1, 2, 5, 6, 1}, {0, 3, 2, 4, 1}});
  for (auto& [nums, removeQueries] : tests) {
    auto answer = s.maximumSegmentSum(nums, removeQueries);
    std::copy(answer.begin(), answer.end(), std::ostream_iterator<long long>(std::cout, " "));
    std::cout << '\n';
  }
}
