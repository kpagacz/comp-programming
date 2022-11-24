// link to the problem: https://leetcode.com/problems/word-search/
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

// Runtime: 573 ms, faster than 61.03% of C++ online submissions for Word Search.
// Memory Usage: 8 MB, less than 79.86% of C++ online submissions for Word Search.

class Solution {
 public:
  bool exist(std::vector<std::vector<char>>& board, std::string word) {
    bool found = false;
    for (int i = 0; i < board.size(); ++i)
      for (int j = 0; j < board[0].size(); ++j)
        if (board[i][j] == word[0]) found = found || search(board, i, j, word, 1);
    return found;
  }

 private:
  bool search(std::vector<std::vector<char>>& board, const int& row, const int& col, const std::string& word,
              const int& currentLetter) {
    if (currentLetter == word.size()) return true;
    bool found = false;
    char previousLetter = board[row][col];
    board[row][col] = '$';
    for (const auto& [deltaX, deltaY] : cardinalDirections) {
      if (isValidCell(row + deltaX, col + deltaY, board) && board[row + deltaX][col + deltaY] == word[currentLetter])
        found = found || search(board, row + deltaX, col + deltaY, word, currentLetter + 1);
    }
    board[row][col] = previousLetter;
    return found;
  }

  bool isValidCell(const int& row, const int& col, const std::vector<std::vector<char>>& board) {
    return row >= 0 && row < board.size() && col >= 0 && col < board[0].size();
  }

  const std::vector<std::pair<int, int>> cardinalDirections{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<std::vector<std::vector<char>>, std::string>> cases;
  cases.push_back({{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}}, "ABCCED"});
  cases.push_back({{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}}, "SEE"});
  cases.push_back({{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}}, "ABCB"});
  for (auto [board, word] : cases) std::cout << std::boolalpha << s.exist(board, word) << '\n';
}
