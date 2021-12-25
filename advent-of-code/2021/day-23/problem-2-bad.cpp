// link to the problem https://adventofcode.com/2021/day/23
#include <algorithm>
#include <iostream>
#include <numeric>
#include <queue>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

const std::unordered_map<char, int> char_factor{
    {'A', 1}, {'B', 10}, {'C', 100}, {'D', 1000}};

using state = std::vector<std::string>;
using point = std::pair<int, int>;
struct State {
  state s;
  int energy = 0;
  State() = default;
};

const std::vector<point> cardinals{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
const std::unordered_map<char, std::vector<point>> rooms{
    {'A', {{2, 3}, {3, 3}, {4, 3}, {5, 3}}},
    {'B', {{2, 5}, {3, 5}, {4, 5}, {5, 5}}},
    {'C', {{2, 7}, {3, 7}, {4, 7}, {5, 7}}},
    {'D', {{2, 9}, {3, 9}, {4, 9}, {5, 9}}},
    {'.', {}}};

const std::vector<point> positions{
    {1, 1}, {1, 2}, {1, 4}, {1, 6}, {1, 8}, {1, 10}, {1, 11}, {2, 3},
    {3, 3}, {4, 3}, {5, 3}, {2, 5}, {3, 5}, {4, 5},  {5, 5},  {2, 7},
    {3, 7}, {4, 7}, {5, 7}, {2, 9}, {3, 9}, {4, 9},  {5, 9},
};

const std::set<point> allowed_hall{{1, 1}, {1, 2},  {1, 4}, {1, 6},
                                   {1, 8}, {1, 10}, {1, 11}};

bool is_won(const state& s) {
  bool won = true;
  for (auto c = 'A'; c < 'E'; c++) {
    for (int i = 2; i < 6; i++) {
      auto j = 3 + 2 * (c - 'A');
      won = won && s[i][j] == c;
    }
  }
  return won;
}
void print(const state& s) {
  for (const auto& l : s) std::cout << l << '\n';
}

char get(const state& s, const point& p) { return s[p.first][p.second]; }

auto states_comparator = [](const state& a, const state& b) {
  if (a.size() != b.size()) return a.size() - b.size();
  for (int i = 0; i < a.size(); i++) {
    if (a[i].compare(b[i]) != 0) return (unsigned long)a[i].compare(b[i]);
  }
  return 0UL;
};

auto energy_comparator = [](const State& a, const State& b) {
  return a.energy - b.energy;
};

point add(const point& a, const point& b) {
  return {a.first + b.first, a.second + b.second};
}

bool empty_room_or_ready(char c, const state& g) {
  return std::all_of(rooms.at(c).begin(), rooms.at(c).end(), [&](auto& p) {
    return get(g, p) == '.' || get(g, p) == c;
  });
}

bool is_bottom_destination(const point& src, const point& dest,
                           const state& g) {
  auto room = rooms.at(get(g, src));
  return get(g, dest) == '.' &&
         (dest == room.back() || get(g, add(dest, {1, 0})) == get(g, src));
}

bool is_allowed_move(const point& start, const point& end, const state& g) {
  bool allowed = false;
  if (allowed_hall.contains(start)) {
    allowed = empty_room_or_ready(get(g, start), g) &&
              is_bottom_destination(start, end, g);
  } else {
    allowed = allowed_hall.contains(end);
  }
  return allowed;
}

bool can_move(const point& start, const state& g) {
  auto room = rooms.at(get(g, start));
  if (allowed_hall.contains(start)) return true;
  auto dest_room = rooms.at(get(g, start));
  if (std::find(dest_room.begin(), dest_room.end(), start) == dest_room.end())
    return true;
  return std::find(room.begin(), room.end(), start) != room.end() &&
         !empty_room_or_ready(get(g, start), g);
}

int main(int argc, char** argv) {
  // read input
  std::string line;
  state initial;
  while (std::getline(std::cin, line)) {
    initial.push_back(line);
  }

  // find the winning state
  std::vector<State> won_states;
  State first;
  first.s = initial;
  first.energy = 0;

  std::set<state, decltype(states_comparator)> seen_states;
  std::priority_queue to_see(energy_comparator, std::vector<State>());
  to_see.push(first);

  while (!to_see.empty()) {
    State top = to_see.top();
    to_see.pop();
    // print(top.s);
    if (is_won(top.s)) {
      won_states.push_back(top);
      break;
    }
    if (seen_states.contains(top.s)) continue;
    seen_states.insert(top.s);

    // get reachable coordinates
    for (const auto& start : positions) {
      if (get(top.s, start) == '.') continue;
      if (!can_move(start, top.s)) continue;
      std::set<point> already_reached{start};
      std::vector<point> reachable;

      std::queue<point> to_visit;
      for (const auto& p : cardinals) to_visit.push(add(start, p));

      while (!to_visit.empty()) {
        point front = to_visit.front();
        to_visit.pop();

        if (already_reached.contains(front)) continue;
        already_reached.insert(front);

        if (get(top.s, front) != '.') continue;
        reachable.push_back(front);
        for (const auto& p : cardinals) to_visit.push(add(front, p));
      }

      // std::cout << "Start: " << start.first << " " << start.second << '\n';
      // std::cout << "Reachable: \n";
      reachable.erase(
          std::remove_if(
              reachable.begin(), reachable.end(),
              [&](auto dest) { return !is_allowed_move(start, dest, top.s); }),
          reachable.end());
      for (const auto& p : reachable) {
        State new_state;
        new_state.s = top.s;
        std::swap(new_state.s[start.first][start.second],
                  new_state.s[p.first][p.second]);
        int cost =
            std::abs(p.first - start.first) + std::abs(p.second - start.second);
        cost *= char_factor.at(get(new_state.s, p));
        new_state.energy = top.energy + cost;
        to_see.push(new_state);
      }
    }
  }

  // print result
  std::sort(won_states.begin(), won_states.end(),
            [](auto a, auto b) { return a.energy < b.energy; });

  std::cout << won_states[0].energy << '\n';
}
