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

class Solution {
 public:
  int uniqueMorseRepresentations(std::vector<std::string>& words) {
    const std::vector<std::string> morse{".-",   "-...", "-.-.", "-..",  ".",   "..-.", "--.",  "....", "..",
                                         ".---", "-.-",  ".-..", "--",   "-.",  "---",  ".--.", "--.-", ".-.",
                                         "...",  "-",    "..-",  "...-", ".--", "-..-", "-.--", "--.."};
    std::unordered_set<std::string> seen;
    for (const auto& word : words) {
      std::string encoded;
      for (const auto& letter : word) encoded += morse[letter - 'a'];
      seen.insert(encoded);
    }
    return seen.size();
  }
};

int main(int argc, char** argv) {}
