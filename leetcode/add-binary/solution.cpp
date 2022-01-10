// link to the problem: https://leetcode.com/problems/add-binary/
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
    std::string addBinary(std::string a, std::string b) {
      if (a.size() < b.size()) return addBinary(b, a);
      // a is at least the size of b

      std::string answer = "";
      int carry = 0;
      for(auto i{0}; i < b.size(); i++) {
        int sum = a[a.size() - 1 - i] - '0' + b[b.size() - 1 - i] - '0' + carry;
        answer += std::to_string(sum % 2);
        carry = sum / 2;
      }
      for (auto i{0}; i < a.size() - b.size();i++) {
        int sum = (a[a.size() - b.size() - 1 - i] - '0' + carry);
        answer += std::to_string(sum % 2);
        carry = sum / 2;
      }
      if (carry) answer += "1";
      std::reverse(answer.begin(), answer.end());
      return answer;
    }
};

int main(int argc, char** argv) {
  Solution s;
  std::cout << s.addBinary("0", "0");
}
