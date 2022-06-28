// link to the problem: https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/
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

// That's correct but also super slow
// Runtime: 205 ms, faster than 10.33% of C++ online submissions for Minimum Deletions to Make Character Frequencies
// Unique. Memory Usage: 17.4 MB, less than 37.46% of C++ online submissions for Minimum Deletions to Make Character
// Frequencies Unique.

class Solution {
 public:
  int minDeletions(std::string s) {
    std::unordered_map<char, int> characterFrequencies;
    for (const auto& c : s)
      if (const auto& [ptr, inserted] = characterFrequencies.try_emplace(c, 1); !inserted) characterFrequencies[c]++;

    // for (const auto& [c, frequency] : characterFrequencies)
    //   std::cout << "Char: " << c << " Freq: " << frequency << '\n';

    std::unordered_map<int, int> fFrequencies;
    for (const auto& [c, frequency] : characterFrequencies)
      if (const auto [ptr, inserted] = fFrequencies.try_emplace(frequency, 1); !inserted) fFrequencies[frequency]++;
    // for (const auto& [c, frequency] : fFrequencies) std::cout << "Frequency: " << c << " Freq: " << frequency <<
    // '\n';

    // the below will not throw because s is at least one element long
    auto [max, _] = *std::max_element(fFrequencies.begin(), fFrequencies.end());
    int maxFrequency = max;
    // std::cout << "Max frequency: " << maxFrequency << '\n';

    int deletedCharacters = 0;
    while (maxFrequency > 0) {
      if (fFrequencies.count(maxFrequency) == 0) {
        --maxFrequency;
        continue;
      }
      auto difference = fFrequencies[maxFrequency] - 1;
      maxFrequency--;
      deletedCharacters += difference;
      if (difference == 0) continue;
      if (fFrequencies.find(maxFrequency) == fFrequencies.end())
        fFrequencies[maxFrequency] = difference;
      else
        fFrequencies[maxFrequency] += difference;
    }
    return deletedCharacters;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << s.minDeletions("zzzzzzzzzzzzzzzza");
}
