#include <fstream>
#include <iostream>
#include <string>
#include <vector>

class Solution {
 public:
  std::size_t detect_unique(const std::string& input, const int& length) {
    for (auto it = input.begin() + 3; it != input.end(); it++) {
      std::vector<bool> previous(26, false);
      bool allUnique = true;
      for (int i = 0; i < length; i++)
        if (previous[*(it - i)]) allUnique = false;
        else previous[*(it - i)] = true;

      if (allUnique) return it - input.begin() + 1;
    }
    return -1;
  }
};

int main() {
  std::fstream file("input", std::ios_base::in);
  std::string buffer;
  file >> buffer;
  Solution s;
  std::cout << "Part 1: " << s.detect_unique(buffer, 4) << '\n';
  std::cout << "Part 2: " << s.detect_unique(buffer, 14) << '\n';
  return 0;
}
