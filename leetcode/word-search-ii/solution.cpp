// link to the problem: https://leetcode.com/problems/word-search-ii/
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

// Runtime: 1382 ms, faster than 44.99% of C++ online submissions for Word Search II.
// Memory Usage: 15.8 MB, less than 19.42% of C++ online submissions for Word Search II.

class Solution {
 public:
  std::vector<std::string> findWords(std::vector<std::vector<char>>& board, std::vector<std::string>& words) {
    this->trie = new Node();
    this->board = board;
    for (const auto& word : words) addToTrie(word);
    std::vector<std::string> answer;
    for (int i = 0; i < board.size(); i++)
      for (int j = 0; j < board[0].size(); j++) {
        // auto resetBoard = this->board;
        std::string word;
        if (trie->isAChild(board[i][j])) dfs(i, j, answer, trie, board, word);
      }
    return answer;
  }

 private:
  struct Node {
    std::vector<Node*> children;
    bool end = false;
    constexpr static const int alphabet = 26;
    Node() { this->children = std::vector<Node*>(alphabet); };
    bool isAChild(const char& c) {
      if (c == '$')
        return false;
      else
        return children[c - 'a'] != nullptr;
    }
    void addChild(const char& c, Node* child) { children[c - 'a'] = child; }
    Node* getChild(const char& c) { return children[c - 'a']; }
  };
  Node* trie;
  std::vector<std::vector<char>> board;
  const std::vector<std::pair<int, int>> cardinalDirections{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

  void addToTrie(const std::string& word) {
    Node* it = trie;
    for (const auto& c : word) {
      if (!it->isAChild(c)) it->addChild(c, new Node());
      it = it->getChild(c);
    }
    it->end = true;
  }

  bool areCoordsGood(const int& row, const int& col) {
    return row >= 0 && row < board.size() && col >= 0 && col < board[0].size();
  }

  void dfs(const int& row, const int& col, std::vector<std::string>& foundWords, Node* it,
           std::vector<std::vector<char>>& board, std::string& wordSoFar) {
    // std::cout << "row: " << row << " col: " << col << " it->word: " << it->word << '\n';
    // std::cout << board[row][col] << '\n';
    it = it->getChild(board[row][col]);
    wordSoFar += board[row][col];
    char previous = board[row][col];
    board[row][col] = '$';
    if (it->end) {
      foundWords.push_back(wordSoFar);
      it->end = false;
    }
    for (const auto& [rowDelta, colDelta] : cardinalDirections)
      if (areCoordsGood(row + rowDelta, col + colDelta) && it->isAChild(board[row + rowDelta][col + colDelta]))
        dfs(row + rowDelta, col + colDelta, foundWords, it, board, wordSoFar);
    board[row][col] = previous;
    wordSoFar.pop_back();
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::pair<std::vector<std::vector<char>>, std::vector<std::string>>> cases;
  cases.push_back({{{'o', 'a', 'a', 'n'}, {'e', 't', 'a', 'e'}, {'i', 'h', 'k', 'r'}, {'i', 'f', 'l', 'v'}},
                   {"oath", "pea", "eat", "rain", "oathf"}});
  cases.push_back({{{'o', 'a', 'a', 'n'}, {'e', 't', 'a', 'e'}, {'i', 'h', 'k', 'r'}, {'i', 'f', 'l', 'v'}},
                   {"oath", "pea", "eat", "rain", "hklf", "hf"}});
  for (auto [board, words] : cases) {
    auto answer = s.findWords(board, words);
    std::copy(answer.begin(), answer.end(), std::ostream_iterator<std::string>(std::cout, " "));
    std::cout << '\n';
  }
}
