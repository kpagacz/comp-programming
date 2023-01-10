#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

using Num = int64_t;
class Solution {
 public:
  Num part1(const std::string& path) {
    auto instructions = parseInput(path);
    auto currentDirection = 0;
    Point currentPosition = {0, 0};

    auto executeInstruction = [&](const auto& instruction) {
      // std::cout << instruction.first << " " << instruction.second << '\n';
      switch (instruction.first) {
        case 'F': {
          auto direction = directions.at(currentDirection);
          auto times = instruction.second;
          while (times--) currentPosition.first += direction.first, currentPosition.second += direction.second;
          break;
        }
        case 'L': {
          const auto direction = instruction.first == 'L' ? -1 : 1;
          const auto times = instruction.second / 90;
          const auto shift = direction * times;
          currentDirection += shift;
          while (currentDirection < 0) currentDirection += directions.size();
          currentDirection %= directions.size();
          break;
        }
        case 'R': {
          const auto direction = instruction.first == 'L' ? -1 : 1;
          const auto times = instruction.second / 90;
          const auto shift = direction * times;
          currentDirection += shift;
          while (currentDirection < 0) currentDirection += directions.size();
          currentDirection %= directions.size();
          break;
        }
        default: {
          auto direction = directions.at(compassToDirection.at(instruction.first));
          auto times = instruction.second;
          while (times--) currentPosition.first += direction.first, currentPosition.second += direction.second;
          break;
        }
      }
    };
    std::ranges::for_each(instructions, executeInstruction);

    return std::abs(currentPosition.first) + std::abs(currentPosition.second);
  }

  Num part2(const std::string& path) {
    auto instructions = parseInput(path);
    Point waypoint = {10, 1};
    Point ship = {0, 0};

    auto executeInstruction = [&](const auto& instruction) {
      // std::cout << instruction.first << " " << instruction.second << '\n';
      switch (instruction.first) {
        case 'N':
        case 'S':
        case 'E':
        case 'W': {
          auto direction = directions.at(compassToDirection.at(instruction.first));
          auto times = instruction.second;
          waypoint.first += times * direction.first, waypoint.second += times * direction.second;
          break;
        }
        case 'R':
        case 'L': {
          if (instruction.second == 180) waypoint.first = -waypoint.first, waypoint.second = -waypoint.second;
          if ((instruction.first == 'R' && instruction.second == 90) ||
              (instruction.first == 'L' && instruction.second == 270)) {
            auto oldX = waypoint.first;
            waypoint.first = waypoint.second;
            waypoint.second = -oldX;
          }
          if ((instruction.first == 'L' && instruction.second == 90) ||
              (instruction.first == 'R' && instruction.second == 270)) {
            auto oldX = waypoint.first;
            waypoint.first = -waypoint.second;
            waypoint.second = oldX;
          }
          break;
        }
        case 'F': {
          auto times = instruction.second;
          ship.first += times * waypoint.first, ship.second += times * waypoint.second;
          break;
        }
      }
    };

    std::ranges::for_each(instructions, executeInstruction);

    return std::abs(ship.first) + std::abs(ship.second);
  }

 private:
  using Point = std::pair<Num, Num>;
  const std::vector<Point> directions{{1, 0}, {0, -1}, {-1, 0}, {0, 1}};
  const std::unordered_map<char, int> compassToDirection{{'E', 0}, {'S', 1}, {'W', 2}, {'N', 3}};
  using Instruction = std::pair<char, Num>;
  std::vector<Instruction> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    std::vector<Instruction> instructions;
    while (input >> line) {
      Num num = std::stoull(line.substr(1));
      instructions.push_back({line[0], num});
    }
    return instructions;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
