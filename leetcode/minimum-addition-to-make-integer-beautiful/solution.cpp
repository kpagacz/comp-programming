// link to the problem: https://leetcode.com/problems/minimum-addition-to-make-integer-beautiful/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Minimum Addition to Make Integer Beautiful.
// Memory Usage: 6 MB, less than 30.04% of C++ online submissions for Minimum Addition to Make Integer Beautiful.

class Solution {
 public:
  long long makeIntegerBeautiful(long long n, int target) {
    std::string number = std::to_string(n);
    std::string answer = "";
    int sum = std::accumulate(number.begin(), number.end(), 0, [](auto sum, auto el) { return sum += el - '0'; });
    bool addOne = false;
    for (int i = number.size() - 1; i >= 0 && sum > target; i--) {
      if (addOne) number[i]++;
      sum -= ((number[i] - '0'));
      answer += ((10 - (number[i] - '0')) % 10) + '0';
      addOne = number[i] != '0';
      sum += addOne;
    }
    std::reverse(answer.begin(), answer.end());
    return answer == "" ? 0 : std::stoll(answer);
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<int64_t, int>> cases;
  cases.push_back({16, 6});
  cases.push_back({467, 6});
  cases.push_back({999999999999999, 2});
  cases.push_back({1, 1});
  cases.push_back({19, 1});
  cases.push_back({590, 1});
  cases.push_back({70616, 5});

  for (auto [n, target] : cases) std::cout << s.makeIntegerBeautiful(n, target) << '\n';
}
