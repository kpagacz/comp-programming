// link to the problem: https://leetcode.com/problems/count-vowels-permutation/
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

// Runtime: 206 ms, faster than 21.49% of C++ online submissions for Count Vowels Permutation.
// Memory Usage: 51.7 MB, less than 14.48% of C++ online submissions for Count Vowels Permutation.

// We could optimize by using only two arrays of int...

constexpr int MODULO = 1e9 + 7;

class Solution {
 public:
  int countVowelPermutation(int n) {
    std::vector<std::vector<int>> permutation(n + 2, std::vector<int>(26, 0));
    permutation[1]['a' - 'a'] = permutation[1]['e' - 'a'] = permutation[1]['i' - 'a'] = permutation[1]['o' - 'a'] =
        permutation[1]['u' - 'a'] = 1;
    for (int i = 2; i <= n; ++i) {
      permutation[i]['e' - 'a'] = (permutation[i]['e' - 'a'] + permutation[i - 1]['a' - 'a']) % MODULO;
      for (const auto& sufix : {'a', 'i'})
        permutation[i][sufix - 'a'] = (permutation[i][sufix - 'a'] + permutation[i - 1]['e' - 'a']) % MODULO;
      for (const auto& sufix : {'a', 'e', 'o', 'u'})
        permutation[i][sufix - 'a'] = (permutation[i][sufix - 'a'] + permutation[i - 1]['i' - 'a']) % MODULO;
      for (const auto& sufix : {'i', 'u'})
        permutation[i][sufix - 'a'] = (permutation[i][sufix - 'a'] + permutation[i - 1]['o' - 'a']) % MODULO;
      permutation[i]['a' - 'a'] = (permutation[i]['a' - 'a'] + permutation[i - 1]['u' - 'a']) % MODULO;
    }

    int answer = 0;
    for (const auto& num : permutation[n]) answer = (answer + num) % MODULO;
    return answer;
  }
};

int main(int argc, char** argv) {}
