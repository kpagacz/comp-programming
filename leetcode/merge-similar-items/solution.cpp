// link to the problem: https://leetcode.com/problems/merge-similar-items/
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

// Runtime: 41 ms, faster than 100.00% of C++ online submissions for Merge Similar Items.
// Memory Usage: 15.6 MB, less than 100.00% of C++ online submissions for Merge Similar Items.

class Solution {
 public:
  std::vector<std::vector<int>> mergeSimilarItems(std::vector<std::vector<int>>& items1,
                                                  std::vector<std::vector<int>>& items2) {
    auto comp = [](const std::vector<int>& lhs, const std::vector<int>& rhs) {
      return std::less<int>()(lhs[0], rhs[0]);
    };

    std::sort(items1.begin(), items1.end(), comp);
    std::sort(items2.begin(), items2.end(), comp);

    std::vector<std::vector<int>> answer;
    int items1It = 0, items2It = 0;
    while (items1It < items1.size() && items2It < items2.size()) {
      if (items1[items1It][0] == items2[items2It][0]) {
        answer.push_back({items1[items1It][0], items1[items1It][1] + items2[items2It][1]});
        items1It++;
        items2It++;
      } else if (items1[items1It][0] < items2[items2It][0]) {
        answer.push_back(items1[items1It]);
        items1It++;
      } else {
        answer.push_back(items2[items2It]);
        items2It++;
      }
    }

    if (items1It == items1.size()) {
      std::copy(items2.begin() + items2It, items2.end(), std::back_inserter(answer));
    } else {
      std::copy(items1.begin() + items1It, items1.end(), std::back_inserter(answer));
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
