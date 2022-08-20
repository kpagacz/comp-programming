// link to the problem: https://leetcode.com/problems/count-special-integers/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Count Special Integers.
// Memory Usage: 6.1 MB, less than 36.49% of C++ online submissions for Count Special Integers.

// Explanation:

//     Transform N + 1 to arrayList
//     Count the number with digits < n
//     Count the number with same prefix

// For example,
// if N = 8765, L = [8,7,6,6],
// the number without repeated digit can the the following format:
// XXX
// XX
// X
// 1XXX ~ 7XXX
// 80XX ~ 86XX
// 870X ~ 875X
// 8760 ~ 8765

// Complexity:

// The permutations A(m,n) can improved to O(1).
// We count digit by digit, so
// Time is O(logN * 10)
// Space is O(logN)

class Solution {
 public:
  int countSpecialNumbers(int n) {
    n++;
    std::string number = std::to_string(n);
    int digits = number.size();

    int answer = 0;
    for (int i = 1; i < digits; ++i) answer += 9 * partialPermutation(9, i - 1);

    std::vector<bool> seen(10, false);
    for (int i = 0; i < digits; i++) {
      for (int j = i > 0 ? 0 : 1; j < number[i] - '0'; ++j)
        if (seen[j] == false) answer += partialPermutation(9 - i, digits - i - 1);

      if (seen[number[i] - '0'] == true) return answer;
      seen[number[i] - '0'] = true;
    }
    return answer;
  }

  int partialPermutation(int total, int elements) {
    if (elements == 0) return 1;
    int answer = 1;
    while (elements--) answer *= total--;
    return answer;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<int> tests{3121, 135, 100, 4131};
  for (auto& test : tests) std::cout << test << " no: " << s.countSpecialNumbers(test) << '\n';
}
