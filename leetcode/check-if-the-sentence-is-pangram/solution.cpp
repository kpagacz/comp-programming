// link to the problem: https://leetcode.com/problems/check-if-the-sentence-is-pangram/
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

// Runtime: 3 ms, faster than 80.43% of C++ online submissions for Check if the Sentence Is Pangram.
// Memory Usage: 6.5 MB, less than 55.40% of C++ online submissions for Check if the Sentence Is Pangram.

class Solution {
 public:
  bool checkIfPangram(std::string sentence) {
    bool* seen = new bool[26];
    std::fill(seen, seen + 26, false);
    int counter = 0;
    for (char& c : sentence) {
      if (!seen[c - 'a']) {
        seen[c - 'a'] = true;
        counter++;
        if (counter == 26) {
          delete[] seen;
          return true;
        }
      }
    }

    delete[] seen;
    return false;
  }
};

int main(int argc, char** argv) {}
