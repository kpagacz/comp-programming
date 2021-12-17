#include<math.h>
#include<iostream>
#include<stdlib.h>
#include<vector>
#include<set>

bool lands_in_target(int init_x, int init_y, int tx1, int tx2, int ty1, int ty2) {
  std::set<int> possible_x_ticks;

  int x_it = 0, ticks = 0;
  int total_x = init_x * (init_x + 1) / 2;
  bool stationary_x = total_x >= tx1 && total_x <= tx2;
  while (init_x >= 0)
  {
    ++ticks;
    x_it += init_x;
    if (x_it >= tx1 && x_it <= tx2)
      possible_x_ticks.insert(ticks);
    --init_x;
  }

  int max_x_tick = *std::max_element(possible_x_ticks.begin(), possible_x_ticks.end());
  ticks = 0;
  int y_it = 0;
  while(true) {
    ++ticks;
    y_it += init_y;
    if (y_it >= ty1 && y_it <= ty2 && (possible_x_ticks.find(ticks) != possible_x_ticks.end() || (stationary_x && ticks > max_x_tick)))
      return true;

    if (init_y < 0 && y_it < ty1)
      break;
    --init_y;
  }

  return false;
}

int main(int argc, char** argv) {
  int x1, x2, y1, y2;
  std::scanf("target area: x=%d..%d , y=%d..%d", &x1, &x2, &y1, &y2);

  std::vector<int> possible_x, possible_y;
  for (int i = 0; i <= x2; i++)
    possible_x.push_back(i);

  for (int i = y1; i <= -y1 - 1; i++)
    possible_y.push_back(i);

  int counter = 0;
  for(const auto& x : possible_x) for(const auto& y : possible_y) if(lands_in_target(x, y, x1, x2, y1, y2))
        counter++;

  std::cout << "Possible initial configurations: " << counter << '\n';
}
