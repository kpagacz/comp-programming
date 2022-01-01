// link to the problem: https://leetcode.com/problems/defanging-an-ip-address/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <regex>
#include <sstream>
#include <string>
#include <vector>

class Solution {
 public:
  std::string defangIPaddr(std::string address) {
    return std::regex_replace(address, std::regex("\\.") , "[.]");
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << s.defangIPaddr("255.100.50.0");
}
