// link to the problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
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
  std::vector<std::string> letterCombinations(std::string digits) {
    if (digits.size() == 0) return {};
    std::vector<std::string> answer;

    std::string currentCombination = "";
    letterCombination(answer, digits, 0, currentCombination);
    return answer;
  }

  void letterCombination(std::vector<std::string>& answer, const std::string& digits, int currentDigit,
                         std::string& currentCombination) {
    if (currentDigit >= digits.size()) {
      answer.push_back(currentCombination);
      return;
    }

    const auto& phoneDigit = digits[currentDigit];
    for (const auto& letter : phoneDigitMapping.at(phoneDigit)) {
      currentCombination += letter;
      letterCombination(answer, digits, currentDigit + 1, currentCombination);
      currentCombination.pop_back();
    }
  }

 private:
  const std::unordered_map<char, std::vector<std::string>> phoneDigitMapping{
      {'2', {"a", "b", "c"}}, {'3', {"d", "e", "f"}},      {'4', {"g", "h", "i"}}, {'5', {"j", "k", "l"}},
      {'6', {"m", "n", "o"}}, {'7', {"p", "q", "r", "s"}}, {'8', {"t", "u", "v"}}, {'9', {"w", "x", "y", "z"}}};
};

int main(int argc, char** argv) {}
