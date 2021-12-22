#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <numeric>
constexpr int P1START = 4;
constexpr int P2START = 5;
constexpr int WIN = 1000;
int main(int argc, char** argv) {
  int die = 0;
  int p1_score = 0;
  int p2_score = 0;
  int p1_pos = P1START, p2_pos = P2START;
  bool turn = true;
  int rolls = 0;
  while (p1_score < 1000 && p2_score < 1000) {
    std::cout << p1_score << " " << p2_score << '\n';
    if (turn) {
      for (int i = 0; i < 3; i++) {
        p1_pos += (die++) + 1;
        die %= 100;
        rolls++;
      }
      p1_score += (p1_pos % 10) + 1;
    } else {
      for (int i = 0; i < 3; i++) {
        p2_pos += die++ + 1;
        die %= 100;
        rolls++;
      }
      p2_score += (p2_pos % 10) + 1;
    }
    turn = !turn;
  }

  int lower_player_score = std::min(p1_score, p2_score);

  std::cout << "Lower player score * rolls to win: " << lower_player_score
            << " * " << rolls << " = "
            << rolls * lower_player_score << '\n';
}
