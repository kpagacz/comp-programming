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
  int estimated_energy = 0;
  std::vector<const state*> previous_states;
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
  return b.estimated_energy < a.estimated_energy;
};

point add(const point& a, const point& b) {
  return {a.first + b.first, a.second + b.second};
}

bool empty_room_or_ready(char c, const state& g) {
  return std::all_of(rooms.at(c).begin(), rooms.at(c).end(), [&](auto& p) {
    return get(g, p) == '.' || get(g, p) == c;
  });
}

bool can_move(const point& start, const state& g) {
  auto room = rooms.at(get(g, start));
  if (allowed_hall.contains(start)) return true;
  auto dest_room = rooms.at(get(g, start));
  auto found = std::find(dest_room.begin(), dest_room.end(), start);
  if (found == dest_room.end()) return true;
  auto frog_type = get(g, start);
  bool answer = true;
  while (found != dest_room.end()) {
    answer = answer && get(g,*found) == frog_type;
    found++;
  }
    return !answer;
}

point find_room_front(const point& src, const state& g) {
  auto room = rooms.at(get(g, src));
  auto found = std::find_if(room.begin(), room.end(),
                            [&](auto p) { return get(g, p) != '.'; });
  if (found == room.end()) return room.back();
  return {found->first - 1, found->second};
}

const std::vector<point> room_entries{{1, 3}, {1, 5}, {1, 7}, {1, 9}};

int distance(const point& a, const point& b) {
  return std::abs(a.first - b.first) + std::abs(a.second - b.second);
}

int estimated_cost_to_win(const state& g) {
  int answer = 0;
  std::vector<int> frogs_entered(4, 0);
  std::vector<int> frogs_needed_to_make_space(4, 0);
  for (const auto& p : positions) {
    if (get(g, p) == '.') continue;
    if (!can_move(p, g)) continue;
    auto frog = get(g, p) - 'A';
    auto entry = room_entries[frog];
    auto room = rooms.at(get(g, p));
    if (std::find(room.begin(), room.end(), p) != room.end())
      frogs_needed_to_make_space[frog]++;
    frogs_entered[frog]++;
    answer += distance(entry, p) * char_factor.at(get(g, p));
  }
  for (int i = 0; i < 4; i++) {
    auto factor = char_factor.at((char)('A' + i));
    answer += (frogs_entered[i] * (frogs_entered[i] + 1) / 2) * factor;
    answer += factor * (frogs_needed_to_make_space[i]);
  }
  return answer;
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
    print(top.s);

    if (is_won(top.s)) {
      won_states.push_back(top);
      break;
    }

    if (seen_states.contains(top.s)) continue;
    auto inserted = seen_states.insert(top.s);
    top.previous_states.push_back(&*inserted.first);

    for (const auto& start : positions) {
      if (get(top.s, start) == '.') continue;
      if (!can_move(start, top.s)) continue;
      // get reachable coordinates
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

      // pruning reachable
      if (allowed_hall.contains(start)) {
        if (empty_room_or_ready(get(top.s, start), top.s)) {
          auto room_front = find_room_front(start, top.s);
          reachable.erase(
              std::remove_if(reachable.begin(), reachable.end(),
                             [&](auto p) { return p != room_front; }),
              reachable.end());
        } else {
          reachable.clear();
        }
      } else {
        reachable.erase(
            std::remove_if(reachable.begin(), reachable.end(),
                           [&](auto p) { return !allowed_hall.contains(p); }),
            reachable.end());
      }

      // creating new states from valid destinations
      for (const auto& p : reachable) {
        State new_state;
        new_state.s = top.s;
        new_state.previous_states = top.previous_states;
        std::swap(new_state.s[start.first][start.second],
                  new_state.s[p.first][p.second]);
        int cost =
            std::abs(p.first - start.first) + std::abs(p.second - start.second);
        cost *= char_factor.at(get(new_state.s, p));
        new_state.energy = top.energy + cost;
        new_state.estimated_energy =
            new_state.energy + estimated_cost_to_win(new_state.s);
        to_see.push(new_state);
      }
    }
  }

  // print result
  for (auto i : won_states[0].previous_states) {
    print(*i);
  }
  std::cout << won_states[0].energy << '\n';
}
