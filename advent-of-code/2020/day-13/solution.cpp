#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <vector>

#include "utils.cc"

using NumType = int64_t;
class Solution {
 public:
  NumType part1(const std::string& path) {
    auto [departingTime, buses] = parseInput(path);

    auto isX = [](auto s) { return s == "x"; };
    auto castToNum = [](auto s) { return std::stoll(s); };
    auto waitTime = [&](auto interval) {
      return std::pair<NumType, NumType>{
          ((departingTime % interval == 0 ? departingTime / interval : departingTime / interval + 1) * interval) -
              departingTime,
          interval};
    };
    auto min = [](const auto& first, const auto& second) {
      if (first.first < second.first) return first;
      else return second;
    };

    std::vector<NumType> numBuses;
    for (auto bus : buses | std::views::filter(std::not_fn(isX)) | std::views::transform(castToNum))
      numBuses.push_back(bus);

    auto [waitingTime, earliestBus] = std::transform_reduce(numBuses.begin(), numBuses.end(),
                                                            std::pair<NumType, NumType>{INT32_MAX, 0}, min, waitTime);
    return waitingTime * earliestBus;
  }

  NumType part2(const std::string& path) {
    auto [_, buses] = parseInput(path);

    // calculate remainders modulo the buses
    std::vector<NumType> remainders;
    for (auto id : std::views::iota(0u, buses.size())) {
      if (buses[id] == "x") {
        remainders.push_back(-1);
        continue;
      }
      NumType coprime = std::stoull(buses[id]);
      int remainder = -id;
      while (remainder < 0) remainder += coprime;
      remainders.push_back(remainder);
    }

    // filter out "x"s from buses and cast to Num
    auto isX = [](auto s) { return s == "x"; };
    auto castToNum = [](auto s) { return std::stoll(s); };
    std::vector<NumType> numBuses;
    for (const auto& bus : buses | std::views::filter(std::not_fn(isX)) | std::views::transform(castToNum))
      numBuses.push_back(bus);

    // filter out -1s from remainders
    std::erase_if(remainders, [](const auto& remainder) { return remainder == -1; });

    auto upperBound = std::accumulate(buses.begin(), buses.end(), 1ull, [](const auto& lcm, const auto& bus) {
      if (bus == "x") return lcm;
      else return lcm * std::stoull(bus);
    });

    assert(numBuses.size() == remainders.size());

    using Congruence = std::pair<NumType, NumType>;
    auto chineseRemainder = [&](const Congruence& firstCongruence, const Congruence& secondCongruence) {
      const auto& [firstRemainder, firstModulo] = firstCongruence;
      const auto& [secondRemainder, secondModulo] = secondCongruence;
      auto solution = firstRemainder;
      while (solution % secondModulo != secondRemainder) {
        if (INT64_MAX - firstModulo > solution) solution += firstModulo;
        else throw std::out_of_range("Solution out of range");
      }
      assert(solution % firstModulo == firstRemainder && solution % secondModulo == secondRemainder);
      return std::make_pair(solution, firstModulo * secondModulo);
    };

    Congruence solution{0, 1};  // it's an identity element, x == 0 mod 1 is true for all integer x
    std::vector<Congruence> congruences;
    for (auto i : std::views::iota(0u, numBuses.size()))
      congruences.push_back(std::make_pair(remainders[i], numBuses[i]));

    solution = std::accumulate(congruences.begin(), congruences.end(), solution, chineseRemainder);
    while (solution.first < 0) solution.first += solution.second;

    return solution.first % solution.second;
  }

 private:
  std::pair<NumType, std::vector<std::string>> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);

    NumType earliestDeparture;
    input >> earliestDeparture;

    std::string times;
    input >> times;

    return {earliestDeparture, utils::split(times, ",")};
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
