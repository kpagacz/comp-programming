#include <string>
#include<unordered_map>

class Solution {
public:
  std::unordered_map<char, int> values{
      {'I', 1},
      {'V', 5},
      {'X', 10},
      {'L', 50},
      {'C', 100},
      {'D', 500},
      {'M', 1000},
      {'s', 0}
  };

  int romanToInt(std::string s) {
    char prev = 's', curr;
    int total = 0;
    for (int i = 0; i < s.size(); i++) {
      curr = s[i];
      total += values.at(curr);
      if (values.at(prev) < values.at(curr)) total -= 2 * values.at(prev);
      prev = curr;
    }

    return total;
  }
};
