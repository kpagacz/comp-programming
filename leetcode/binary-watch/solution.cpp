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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Binary Watch.
// Memory Usage: 6.5 MB, less than 62.82% of C++ online submissions for Binary Watch.

constexpr int HOUR_BITS = 4, MINUTE_BITS = 6;

class Solution {
 public:
  std::vector<std::string> readBinaryWatch(int turnedOn) {
    std::vector<std::string> answer;
    std::string watch;
    for (int i = 0; i < HOUR_BITS + MINUTE_BITS - turnedOn; ++i) watch += "0";
    for (int i = 0; i < turnedOn; i++) watch += "1";
    do {
      int hours = std::stoi(watch.substr(0, HOUR_BITS), nullptr, 2);
      int minutes = std::stoi(watch.substr(HOUR_BITS, MINUTE_BITS), nullptr, 2);
      if (hours > 11 || minutes > 59) continue;
      std::string time;
      time += std::to_string(hours);
      time += ":";
      if (minutes > 9) time += std::to_string(minutes);
      else time += "0" + std::to_string(minutes);
      answer.push_back(time);
    } while (std::next_permutation(watch.begin(), watch.end()));

    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
}
