// link to the problem: https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
#include<iostream>
#include<vector>
#include<array>
#include<string>
#include<algorithm>
#include<numeric>
#include<sstream>
#include<iterator>
#include<queue>

class Solution {
public:
    int minPartitions(std::string n) {
      auto found = std::max_element(n.begin(), n.end());
      return *found - '0';
    }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << s.minPartitions("27346209830709182346");
}
