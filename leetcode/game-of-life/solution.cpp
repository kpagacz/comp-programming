// link to the problem: https://leetcode.com/problems/game-of-life/
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
  void gameOfLife(std::vector<std::vector<int>>& board) {
    rowsCount = board.size();
    colsCount = board[0].size();

    std::vector<std::vector<int>> newBoard(board);
    for (auto i{0}; i < rowsCount; i++)
      for (auto j{0}; j < colsCount; j++) {
        point p = {i, j};
        int liveNeighboursCount;
        int ns = countLiveNeighbours(p, board);
        switch (board[i][j]) {
          case 0:
            if (countLiveNeighbours(p, board) == 3) newBoard[i][j] = 1;
            else newBoard[i][j] = 0;
            break;
          case 1:
            liveNeighboursCount = countLiveNeighbours(p, board);
            if (liveNeighboursCount < 2 || liveNeighboursCount > 3) newBoard[i][j] = 0;
            else newBoard[i][j] = 1;
            break;
        }
      }
    board = newBoard;
  }

 private:
  int rowsCount, colsCount;
  using point = std::array<int, 2>;
  std::vector<point> directions{{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1}};

  std::vector<point> getNeighbours(const point& p) {
    std::vector<point> neighbours;
    for (const auto& direction : directions) {
      point neighbour = addPoints(direction, p);
      if (validatePoint(neighbour)) neighbours.push_back(neighbour);
    }
    return neighbours;
  }
  point addPoints(const point& p1, const point& p2) {
    return {p1[0] + p2[0], p1[1] + p2[1]};
  }

  bool validatePoint(const point& p) {
    return p[0] >= 0 && p[1] >= 0 && p[0] < rowsCount && p[1] < colsCount;
  }

  int countLiveNeighbours(const point& p, const std::vector<std::vector<int>>& board) {
    int liveCells{0};
    for (const point& neighbour : getNeighbours(p))
      if (board[neighbour[0]][neighbour[1]] == 1) liveCells++;

    return liveCells;
  }
};

int main(int argc, char** argv) {
  std::vector<std::vector<int>> board{{0, 1, 0}, {0, 0, 1}, {1, 1, 1}, {0, 0, 0}};
  for (const auto& v : board) {
    for (const auto& el : v) std::cout << el << " ";
    std::cout << '\n';
  }
  Solution s;
  s.gameOfLife(board);
  std::cout << "New:\n";
  for (const auto& v : board) {
    for (const auto& el : v) std::cout << el << " ";
    std::cout << '\n';
  }
}
