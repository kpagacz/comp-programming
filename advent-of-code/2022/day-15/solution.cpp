#include <cassert>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
 public:
  const int PART1_ROW = 2e6;
  int part1(const std::string& path) {
    auto notBeacons = getNotBeaconsForPart1(path);
    return countNotBeacons(notBeacons[PART1_ROW]);
  }

  const int LLIMIT = 0;
  const int ULIMIT = 4e6;
  const uint64_t MULTIPLIER = 4e6;
  uint64_t part2(const std::string& path) {
    auto notBeacons = getNotBeacons(path);
    for (auto& [y, row] : notBeacons) {
      auto [didFind, x] = findUndiscoveredBeacon(row);
      if (didFind) return MULTIPLIER * x + y;
    }
    return 0;
  }

 private:
  std::unordered_map<int, std::vector<std::pair<int, int>>> getNotBeacons(const std::string_view& path) {
    const std::string DIGITS = "0123456789-";
    std::fstream input(path, std::ios_base::in);
    std::string line;
    auto distance = [](const int& firstX, const int& firstY, const int& secondX, const int& secondY) {
      return std::abs(secondX - firstX) + std::abs(secondY - firstY);
    };
    std::unordered_map<int, std::vector<std::pair<int, int>>> notBeacons;
    while (std::getline(input, line)) {
      int sensorX, sensorY, beaconX, beaconY;
      std::size_t it{0};
      for (auto coord : {&sensorX, &sensorY, &beaconX, &beaconY}) {
        auto coordStart = line.find_first_of(DIGITS, it);
        auto coordEnd = line.find_first_not_of(DIGITS, coordStart);
        *coord = std::stoi(line.substr(coordStart, coordEnd));
        it = coordEnd;
      }
      auto dist = distance(sensorX, sensorY, beaconX, beaconY);
      for (int i = -dist; i <= dist; i++) {
        if (i + sensorY < LLIMIT || i + sensorY > ULIMIT) continue;
        auto span = dist - std::abs(i);
        notBeacons[sensorY + i].push_back({sensorX - span, sensorX + span});
      }
    }
    return notBeacons;
  }
  std::unordered_map<int, std::vector<std::pair<int, int>>> getNotBeaconsForPart1(const std::string_view& path) {
    const std::string DIGITS = "0123456789-";
    std::fstream input(path, std::ios_base::in);
    std::string line;
    auto distance = [](const int& firstX, const int& firstY, const int& secondX, const int& secondY) {
      return std::abs(secondX - firstX) + std::abs(secondY - firstY);
    };
    std::unordered_map<int, std::vector<std::pair<int, int>>> notBeacons;
    while (std::getline(input, line)) {
      int sensorX, sensorY, beaconX, beaconY;
      std::size_t it{0};
      for (auto coord : {&sensorX, &sensorY, &beaconX, &beaconY}) {
        auto coordStart = line.find_first_of(DIGITS, it);
        auto coordEnd = line.find_first_not_of(DIGITS, coordStart);
        *coord = std::stoi(line.substr(coordStart, coordEnd));
        it = coordEnd;
      }
      auto dist = distance(sensorX, sensorY, beaconX, beaconY);
      for (int i = -dist; i <= dist; i++) {
        if (i + sensorY < LLIMIT || i + sensorY > ULIMIT) continue;
        auto span = dist - std::abs(i);
        if (i + sensorY != beaconY) notBeacons[sensorY + i].push_back({sensorX - span, sensorX + span});
        else if (sensorX > beaconX) notBeacons[sensorY + i].push_back({sensorX - span + 1, sensorX + span});
        else notBeacons[sensorY + i].push_back({sensorX - span, sensorX + span - 1});
      }
    }
    return notBeacons;
  }

  int countNotBeacons(std::vector<std::pair<int, int>>& row) {
    if (row.empty()) return 0;
    int answer = 0;
    std::sort(row.begin(), row.end(), [](const auto& first, const auto& second) {
      if (first.first == second.first) return first.second < second.second;
      else return first.first < second.first;
    });
    int countedBefore = row.front().first;
    int rangeEnd = row.front().second;
    for (const auto& [start, end] : row) {
      if (end <= rangeEnd) continue;
      if (start > rangeEnd + 1) answer += rangeEnd - countedBefore + 1, countedBefore = start, rangeEnd = end;
      else rangeEnd = end;
    }
    answer += rangeEnd - countedBefore + 1;
    return answer;
  }

  std::pair<bool, int> findUndiscoveredBeacon(std::vector<std::pair<int, int>> row) {
    std::sort(row.begin(), row.end(), [](const auto& first, const auto& second) {
      if (first.first == second.first) return first.second < second.second;
      else return first.first < second.first;
    });
    int firstStart = row.front().first;
    if (firstStart > LLIMIT) return {true, LLIMIT};
    int previousEnd = firstStart;
    for (const auto& [start, end] : row) {
      if (start > previousEnd + 1) return {true, start - 1};
      else previousEnd = std::max(previousEnd, end);
    }
    if (previousEnd < ULIMIT) return {true, ULIMIT};
    return {false, -1};
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
