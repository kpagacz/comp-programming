// link to the problem: https://leetcode.com/problems/minimum-hours-of-training-to-win-a-competition/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Minimum Hours of Training to Win a Competition.
// Memory Usage: 11.3 MB, less than 8.33% of C++ online submissions for Minimum Hours of Training to Win a Competition.

class Solution {
 public:
  int minNumberOfHours(int initialEnergy, int initialExperience, std::vector<int>& energy,
                       std::vector<int>& experience) {
    int answer = std::max(std::accumulate(energy.begin(), energy.end(), 0) + 1 - initialEnergy, 0);
    for (const auto& opponentExperience : experience) {
      if (initialExperience <= opponentExperience) {
        answer += opponentExperience + 1 - initialExperience;
        initialExperience = opponentExperience + 1;
      }
      initialExperience += opponentExperience;
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
