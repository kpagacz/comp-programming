#include<iostream>
#include<algorithm>
#include<unordered_map>
#include<vector>
#include<numeric>
#include<string>

constexpr int DAYS = 40;

using rules = std::unordered_map<std::string, std::string>;
using polymer = std::unordered_map<std::string, uint64_t>;

void apply_step(polymer& p, const rules& r) {
  polymer new_polymer_pairs;

  for(auto& pair : p) {
    auto found = r.find(pair.first);
    if (found != r.end()) {
      auto result = new_polymer_pairs.insert({pair.first[0] + found->second, pair.second});
      if (!result.second)
        result.first->second += pair.second; // because the pair may already be in new_polymer_pairs

      result = new_polymer_pairs.insert({found->second + pair.first[1], pair.second});
      if (!result.second)
        result.first->second += pair.second;

      pair.second = 0;
    }
  }

  std::erase_if(p, [](const auto &item)
                { return item.second == 0; });

  for (const auto &pair : new_polymer_pairs)
  {
    auto result = p.insert(pair);
    if (!result.second)
      result.first->second += pair.second; // because the pair may already be in p
  }
}

int main() {
  // read input
  std::string start;
  std::cin >> start;

  polymer p;

  for(int i = 1; i < start.size(); i++) {
    p.insert({start.substr(i - 1, 2), 1});
  }

  rules r;
  std::string from, to;
  while(std::cin >> from) {
    std::cin >> to; // read the arrow
    std::cin >> to; // read the letter
    r.insert({from, to});
  }

  for (auto i = 0U; i < DAYS; i++)
    apply_step(p, r);

  std::vector<uint64_t> ranking('Z' - 'A' + 1, 0);
  for(const auto& pair : p) {
    ranking[pair.first[0] - 'A'] += pair.second;
    ranking[pair.first[1] - 'A'] += pair.second;
  }

  std::erase_if(ranking, [](const auto &el)
                { return el == 0; });

  uint64_t most_common = *std::max_element(ranking.begin(), ranking.end());
  most_common = most_common / 2 + most_common % 2;
  uint64_t least_common = *std::min_element(ranking.begin(), ranking.end());
  least_common = least_common / 2 + least_common % 2;

  std::cout << "Most common: " << most_common << " least common: " << least_common << " diff: " << most_common - least_common << '\n';
}
