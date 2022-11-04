// link to the problem: https://leetcode.com/problems/most-popular-video-creator/
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

// Runtime: 602 ms, faster than 95.54% of C++ online submissions for Most Popular Video Creator.
// Memory Usage: 118.5 MB, less than 97.61% of C++ online submissions for Most Popular Video Creator.

class Solution {
 public:
  std::vector<std::vector<std::string>> mostPopularCreator(std::vector<std::string>& creators,
                                                           std::vector<std::string>& ids, std::vector<int>& views) {
    std::unordered_map<std::string, int> creatorViews;
    std::unordered_map<std::string, std::pair<std::string, int>> creatorMostViewedVideo;
    for (const auto& creator : creators) creatorMostViewedVideo[creator] = {"", -1};

    for (int i = 0; i < creators.size(); ++i) {
      creatorViews[creators[i]] += views[i];
      auto& mostViewedVideo = creatorMostViewedVideo[creators[i]];
      if (mostViewedVideo.second < views[i]) {
        mostViewedVideo.first = ids[i];
        mostViewedVideo.second = views[i];
      } else if (mostViewedVideo.second == views[i] && ids[i] < mostViewedVideo.first) {
        mostViewedVideo.first = ids[i];
        mostViewedVideo.second = views[i];
      }
    }

    int mostViews = 0;
    for (const auto& [creator, views] : creatorViews) mostViews = std::max(mostViews, views);
    std::vector<std::vector<std::string>> answer;
    for (const auto& [creator, views] : creatorViews)
      if (views == mostViews) answer.push_back({creator, creatorMostViewedVideo[creator].first});
    return answer;
  }
};

int main(int argc, char** argv) {}
