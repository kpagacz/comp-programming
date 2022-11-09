// link to the problem: https://leetcode.com/problems/number-of-valid-clock-times/
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

// Runtime: 5 ms, faster than 21.64% of C++ online submissions for Number of Valid Clock Times.
// Memory Usage: 6 MB, less than 7.71% of C++ online submissions for Number of Valid Clock Times.

class Solution {
 public:
  int countTime(std::string time) {
    int answer = 1;
    if (time[0] == time[1] && time[0] == '?') answer *= 24;
    else if (time[0] == '?' && time[1] < '4') answer *= 3;
    else if (time[0] == '?' && time[1] >= '4') answer *= 2;
    else if (time[0] == '2' && time[1] == '?') answer *= 4;
    else if (time[0] < '2' && time[1] == '?') answer *= 10;
    if (time[3] == '?') answer *= 6;
    if (time[4] == '?') answer *= 10;
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::string> cases;
  cases.push_back("?0:00");
  cases.push_back("??:00");
  cases.push_back("?4:00");
  cases.push_back("2?:00");
  cases.push_back("00:??");
  cases.push_back("2?:??");
  for (auto time : cases) std::cout << s.countTime(time) << '\n';
}
