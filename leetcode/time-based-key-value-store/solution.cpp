// link to the problem:
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <map>
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

// Runtime: 770 ms, faster than 18.74% of C++ online submissions for Time Based Key-Value Store.
// Memory Usage: 128.1 MB, less than 70.01% of C++ online submissions for Time Based Key-Value Store.

class TimeMap {
 public:
  TimeMap() {}

  void set(std::string key, std::string value, int timestamp) { map[key].push_back({timestamp, value}); }

  std::string get(std::string key, int timestamp) {
    if (map.count(key) == 0) return "";
    auto lowerBound = std::lower_bound(map[key].begin(), map[key].end(), std::make_pair(timestamp, ""),
                                       std::less<std::pair<int, std::string>>());
    if (lowerBound != map[key].end() && lowerBound->first == timestamp) return lowerBound->second;
    if (std::distance(map[key].begin(), lowerBound) > 0) return (--lowerBound)->second;
    return "";
  }

  using TimestampMap = std::vector<std::pair<int, std::string>>;
  std::unordered_map<std::string, TimestampMap> map;
};

/**
 * Your TimeMap object will be instantiated and called as such:
 * TimeMap* obj = new TimeMap();
 * obj->set(key,value,timestamp);
 * string param_2 = obj->get(key,timestamp);
 */

int main(int argc, char** argv) {
  TimeMap* map = new TimeMap();
  map->set("key", "value", 1);
  std::cout << map->get("key", 2) << '\n';
  std::cout << map->get("key", 1) << '\n';
  map->set("key", "value2", 2);
  std::cout << map->get("key", 2) << '\n';
}
