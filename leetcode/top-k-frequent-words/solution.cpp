// link to the problem: https://leetcode.com/problems/top-k-frequent-words/
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

// Runtime: 27 ms, faster than 45.47% of C++ online submissions for Top K Frequent Words.
// Memory Usage: 12.9 MB, less than 18.22% of C++ online submissions for Top K Frequent Words.

class Solution {
 public:
  std::vector<std::string> topKFrequent(std::vector<std::string>& words, int k) {
    std::unordered_map<std::string, int> positions;
    std::vector<std::pair<int, std::string>> wordsFrequencies;
    for (const auto& word : words) {
      if (positions.count(word) > 0)
        wordsFrequencies[positions[word]].first++;
      else {
        positions[word] = wordsFrequencies.size();
        wordsFrequencies.push_back({1, word});
      }
    }

    std::partial_sort(wordsFrequencies.begin(), wordsFrequencies.begin() + k, wordsFrequencies.end(),
                      [](const auto& first, const auto& second) {
                        return first.first != second.first ? first.first > second.first : first.second < second.second;
                      });
    std::vector<std::string> answer;
    for (int i = 0; i < k; i++) answer.push_back(wordsFrequencies[i].second);
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<std::vector<std::string>, int>> tests;
  tests.push_back({{"i", "love", "leetcode", "i", "love", "coding"}, 2});
  tests.push_back({{"the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"}, 4});
  for (auto [words, k] : tests) {
    auto answer = s.topKFrequent(words, k);
    std::copy(answer.begin(), answer.end(), std::ostream_iterator<std::string>(std::cout, " "));
    std::cout << '\n';
  }
}
