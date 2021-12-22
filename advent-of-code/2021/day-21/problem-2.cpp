#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <numeric>
#include <vector>
constexpr int P1START = 3;
constexpr int P2START = 7;
constexpr int WIN = 21;

void throw_dice(int dice, int p1_pos, int p2_pos, int p1_score, int p2_score,
                bool turn, uint64_t& p1_wins, uint64_t& p2_wins, uint64_t& unis) {
  unis++;
  if (turn) {
    p1_pos += dice + 1;
    p1_score += (p1_pos % 10) + 1;
    if (p1_score >= WIN)
      p1_wins++;
    else {
      throw_dice(0, p1_pos, p2_pos, p1_score, p2_score, !turn, p1_wins,
                 p2_wins, unis);
      throw_dice(1, p1_pos, p2_pos, p1_score, p2_score, !turn, p1_wins,
                 p2_wins, unis);
      throw_dice(2, p1_pos, p2_pos, p1_score, p2_score, !turn, p1_wins,
                 p2_wins, unis);
    }
  } else {
    p2_pos += dice + 1;
    p2_score += (p2_pos % 10) + 1;
    if (p2_score >= WIN)
      p2_wins++;
    else {
      throw_dice(0, p1_pos, p2_pos, p1_score, p2_score, !turn, p1_wins,
                 p2_wins, unis);
      throw_dice(1, p1_pos, p2_pos, p1_score, p2_score, !turn, p1_wins,
                 p2_wins, unis);
      throw_dice(2, p1_pos, p2_pos, p1_score, p2_score, !turn, p1_wins,
                 p2_wins, unis);
    }
  }
}
int main(int argc, char** argv) {
  bool turn = true;
  uint64_t p1_wins = 0, p2_wins = 0, unis = 0;
  throw_dice(0, P1START, P2START, 0, 0, true, p1_wins, p2_wins, unis);
  throw_dice(1, P1START, P2START, 0, 0, true, p1_wins, p2_wins, unis);
  throw_dice(2, P1START, P2START, 0, 0, true, p1_wins, p2_wins, unis);
  std::cout << unis << '\n';
  std::cout << p1_wins << " " << p2_wins << '\n';
}
