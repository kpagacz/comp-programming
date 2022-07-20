// link to the problem: https://leetcode.com/problems/pascals-triangle-ii/
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

class Solution {
 public:
  std::vector<int> getRow(int rowIndex) {
    std::vector<int> answer(rowIndex + 1, 0);
    answer[0] = 1;
    while (rowIndex-- > 0)
      for (int i = answer.size() - 1; i > 0; --i) answer[i] += answer[i - 1];

    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> rows{0, 1, 2, 3, 4};
  for (const auto& row : rows) {
    auto answer = s.getRow(row);
    std::copy(answer.begin(), answer.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
  }
}
