#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <numeric>
#include <vector>
constexpr int P1START = 4;
constexpr int P2START = 5;
constexpr int POS_LIMIT = 200;
using mem =
    std::vector<std::vector<std::vector<std::vector<std::vector<uint64_t>>>>>;

int main(int argc, char** argv) {
  std::array<int, 7> dice_combinations;
  for (int i = 0; i < 3; i++)
    for (int j = 0; j < 3; j++)
      for (int k = 0; k < 3; k++) dice_combinations[i + j + k]++;

  // p1pos - p2pos - p1score - p2score - turn
  mem dp;
  for (int i = 0; i < POS_LIMIT + 10; i++)
    dp.push_back(
        std::vector<std::vector<std::vector<std::vector<uint64_t>>>>());
  for (int i = 0; i < POS_LIMIT + 10; i++)
    for (int j = 0; j < POS_LIMIT + 10; j++)
      dp[i].push_back(std::vector<std::vector<std::vector<uint64_t>>>());

  for (int i = 0; i < POS_LIMIT + 10; i++)
    for (int j = 0; j < POS_LIMIT + 10; j++)
      for (int k = 0; k < 33; k++)
        dp[i][j].push_back(std::vector<std::vector<uint64_t>>());

  for (int i = 0; i < POS_LIMIT + 10; i++)
    for (int j = 0; j < POS_LIMIT + 10; j++)
      for (int k = 0; k < 33; k++)
        for (int l = 0; l < 33; l++)
          dp[i][j][k].push_back(std::vector<uint64_t>());

  for (int i = 0; i < POS_LIMIT + 10; i++)
    for (int j = 0; j < POS_LIMIT + 10; j++)
      for (int k = 0; k < 33; k++)
        for (int l = 0; l < 33; l++)
          for (int m = 0; m < 2; m++) dp[i][j][k][l].push_back({0LL});

  dp[P1START][P2START][0][0][1] = 1;

  for (int i = P1START; i < POS_LIMIT; i++)
    for (int j = P2START; j < POS_LIMIT; j++)
      for (int k = 0; k < 22; k++)
        for (int l = 0; l < 22; l++)
          for (int m = 0; m < 2; m++) {
            if (m == 1) {
              if (l == 21) continue;
              for (auto d = 0; d < dice_combinations.size(); d++) {
                dp[i + d + 3][j][k + (i + d + 3) % 10 + 1][l][0] +=
                    dp[i][j][k][l][m] * dice_combinations[d];
              }
            } else {
              if (k == 21) continue;
              for (auto d = 0; d < dice_combinations.size(); d++) {
                dp[i][j + d + 3][k][l + (j + d + 3) % 10 + 1][1] +=
                    dp[i][j][k][l][m] * dice_combinations[d];
              }
            }
          }

  uint64_t p1_wins = 0;
  for (int i = P1START; i < POS_LIMIT + 10; i++)
    for (int j = P2START; j < POS_LIMIT + 10; j++)
      for (int k = 21; k < 33; k++)
        for (int l = 0; l < 21; l++) {
          p1_wins += dp[i][j][k][l][0];
        }

  uint64_t p2_wins = 0;
  for (int i = P1START; i < POS_LIMIT + 10; i++)
    for (int j = P2START; j < POS_LIMIT + 10; j++)
      for (int k = 0; k < 21; k++)
        for (int l = 21; l < 33; l++) {
          p2_wins += dp[i][j][k][l][1];
        }

  std::cout << "player 1 wins: " << p1_wins << '\n'
            << "player 2 wins: " << p2_wins << '\n';
}
