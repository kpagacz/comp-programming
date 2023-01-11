#include <algorithm>
#include <cassert>
#include <cstdio>
#include <fstream>
#include <functional>
#include <iostream>
#include <numeric>
#include <ranges>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

#include "utils.cc"

using Num = int64_t;
class Solution {
 public:
  Num part1(const std::string& path) {
    const auto [requirements, myTicket, othersTickets] = parseInput(path);

    auto isValueInRequirements = [&](const auto& value) {
      return std::transform_reduce(requirements.begin(), requirements.end(), false, std::logical_or(),
                                   [&](const auto& requirement) { return isValueInRequirement(value, requirement); });
    };

    auto ticketErrorRate = [&](const auto& ticket) {
      return std::transform_reduce(ticket.begin(), ticket.end(), (Num)0, std::plus<Num>(),
                                   [&](const auto& value) { return isValueInRequirements(value) ? 0 : value; });
    };

    return std::transform_reduce(othersTickets.begin(), othersTickets.end(), (Num)0, std::plus<Num>(),
                                 [&](const auto& ticket) { return ticketErrorRate(ticket); });
  }

  Num part2(const std::string& path) {
    auto [requirements, myTicket, othersTickets] = parseInput(path);

    // Filtering invalid tickets
    auto isTicketValid = [&](const auto& ticket) {
      return std::transform_reduce(ticket.begin(), ticket.end(), true, std::logical_and(),
                                   [&](const auto& value) { return isValueInRequirements(value, requirements); });
    };
    std::erase_if(othersTickets, [&](const Ticket& ticket) { return isTicketValid(ticket) == false; });

    auto fulfilledRequirementsForValue = [&](const auto& value) {
      std::set<Num> fulfilledRequirements;
      for (auto idx : std::views::iota(0u, requirements.size())) {
        const auto& requirement = requirements[idx];
        if (isValueInRequirement(value, requirement)) fulfilledRequirements.insert(idx);
      }

      return fulfilledRequirements;
    };
    auto getFulfilledRequirementForPosition = [&](const auto& position) {
      std::set<Num> fulfilledRequirements;
      for (auto idx : std::views::iota(0u, requirements.size())) fulfilledRequirements.insert(idx);

      std::ranges::for_each(othersTickets, [&](const auto& ticket) {
        auto value = ticket[position];
        auto fulfilledRequirementsForPositionInTicket = fulfilledRequirementsForValue(value);
        std::set<Num> newFulfilledRequirements;
        std::ranges::set_intersection(fulfilledRequirements, fulfilledRequirementsForPositionInTicket,
                                      std::inserter(newFulfilledRequirements, newFulfilledRequirements.begin()));
        fulfilledRequirements = newFulfilledRequirements;
      });

      return fulfilledRequirements;
    };

    // known fields map
    std::unordered_map<Num, Num> fieldsMappedToPositions;  // Field idx -> Position in a ticket
    auto removeKnownPositionsFromSet = [&](auto& set) {
      std::erase_if(set, [&](const auto& field) { return fieldsMappedToPositions.contains(field); });
    };

    // Fulfilled requirements per position
    std::vector<std::set<Num>> fulfilledRequirementsForPosition;
    for (auto position : std::views::iota(0u, myTicket.size())) {
      auto fulfilledRequirements = getFulfilledRequirementForPosition(position);
      fulfilledRequirementsForPosition.push_back(fulfilledRequirements);
    }

    // main routine
    while (fieldsMappedToPositions.size() != myTicket.size())
      for (auto position : std::views::iota(0u, myTicket.size())) {
        auto fulfilledRequirements = fulfilledRequirementsForPosition.at(position);
        removeKnownPositionsFromSet(fulfilledRequirements);
        if (fulfilledRequirements.size() == 1) fieldsMappedToPositions[*fulfilledRequirements.begin()] = position;
      }

    const std::set<Num> departureFields{0, 1, 2, 3, 4, 5};
    auto answer = std::transform_reduce(departureFields.begin(), departureFields.end(), (Num)1, std::multiplies<Num>(),
                                        [&](const auto& field) {
                                          auto position = fieldsMappedToPositions.at(field);
                                          return myTicket.at(position);
                                        });
    return answer;
  }

 private:
  using Range = std::pair<Num, Num>;
  using Requirement = std::vector<Range>;
  using Requirements = std::vector<Requirement>;
  using Ticket = std::vector<Num>;

  bool isValueInRequirement(const Num& value, const Requirement& ranges) {
    return std::transform_reduce(ranges.begin(), ranges.end(), false, std::logical_or(),
                                 [&](const auto& range) { return value >= range.first && value <= range.second; });
  }

  bool isValueInRequirements(const Num& value, const Requirements& requirements) {
    return std::transform_reduce(requirements.begin(), requirements.end(), false, std::logical_or(),
                                 [&](const auto& requirement) { return isValueInRequirement(value, requirement); });
  }

  std::tuple<Requirements, Ticket, std::vector<Ticket>> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    Requirements requirements;
    Ticket myTicket;
    std::vector<Ticket> othersTickets;
    std::string mode = "requirements";
    while (std::getline(input, line)) {
      if (mode == "requirements") {
        if (line == "") {
          mode = "myTicket";
          std::getline(input, line);
          continue;
        }
        auto fieldEnd = line.find(':');
        auto fieldName = line.substr(0, fieldEnd);

        auto ranges = line.substr(fieldEnd + 2);
        Range first, second;
        std::sscanf(ranges.c_str(), "%d-%d or %d-%d", &first.first, &first.second, &second.first, &second.second);

        requirements.push_back({first, second});
      } else if (mode == "myTicket") {
        if (line == "") {
          mode = "othersTickets";
          std::getline(input, line);
          continue;
        }
        auto fieldValues = utils::split(line, ",");
        auto castToNum = [&](const auto& stringValue) { return std::stoull(stringValue); };
        std::ranges::transform(fieldValues, std::back_inserter(myTicket), castToNum);
      } else if (mode == "othersTickets") {
        auto fieldValues = utils::split(line, ",");
        auto castToNum = [&](const auto& stringValue) { return std::stoull(stringValue); };
        Ticket newTicket;
        std::ranges::transform(fieldValues, std::back_inserter(newTicket), castToNum);
        othersTickets.push_back(newTicket);
      }
    }

    return {requirements, myTicket, othersTickets};
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
