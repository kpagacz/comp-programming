#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <sstream>
#include <string>
#include <vector>

class Solution {
 public:
  int part1(const std::string& path) {
    auto passports = parseInput(path);

    auto areFieldsValid = [&](const auto& passport) {
      std::vector<std::string> fields;
      for (const auto& fieldValue : passport) fields.push_back(fieldValue.substr(0, fieldValue.find(':')));
      std::sort(fields.begin(), fields.end());
      std::vector<std::string> difference;
      std::set_difference(requiredFields.begin(), requiredFields.end(), fields.begin(), fields.end(),
                          std::back_inserter(difference));
      if (difference.size() > 1) return 0;
      if (difference.size() == 0) return 1;
      if (difference[0] == "cid") return 1;
      return 0;
    };

    return std::transform_reduce(passports.begin(), passports.end(), 0, std::plus<int>(), areFieldsValid);
  }

  int part2(const std::string& path) {
    auto passports = parseInput(path);

    auto areFieldsValid = [&](const auto& passport) {
      std::vector<std::string> fields;
      for (const auto& fieldValue : passport) fields.push_back(fieldValue.substr(0, fieldValue.find(':')));
      std::sort(fields.begin(), fields.end());
      std::vector<std::string> difference;
      std::set_difference(requiredFields.begin(), requiredFields.end(), fields.begin(), fields.end(),
                          std::back_inserter(difference));
      if (difference.size() > 1) return 0;
      if (difference.size() == 0) return 1;
      if (difference[0] == "cid") return 1;
      return 0;
    };

    auto areValuesValid = [&](const auto& passport) {
      for (const auto& fieldValue : passport) {
        int delimiter = fieldValue.find(':');
        std::string field = fieldValue.substr(0, delimiter);
        std::string stringValue = fieldValue.substr(delimiter + 1);
        if (field == "byr") {
          auto value = std::stoi(stringValue);
          if (value < 1920 || value > 2002) return 0;
        }
        if (field == "iyr") {
          auto value = std::stoi(stringValue);
          if (value < 2010 || value > 2020) return 0;
        }
        if (field == "eyr") {
          auto value = std::stoi(stringValue);
          if (value < 2020 || value > 2030) return 0;
        }
        if (field == "hgt") {
          auto unitStart = stringValue.find_first_not_of("0123456789");
          auto value = std::stoi(stringValue.substr(0, unitStart));
          if (unitStart == stringValue.npos) return 0;
          auto unit = stringValue.substr(unitStart);
          if (unit != "cm" && unit != "in") return 0;
          if (unit == "cm" && (value < 150 || value > 193)) return 0;
          if (unit == "in" && (value < 59 || value > 76)) return 0;
        }
        if (field == "hcl") {
          if (stringValue.size() != 7) return 0;
          if (stringValue[0] != '#') return 0;
          if (stringValue.find_first_not_of("0123456789abcdef", 1) != stringValue.npos) return 0;
        }
        if (field == "ecl") {
          const std::vector<std::string> allowedColours{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
          if (std::find(allowedColours.begin(), allowedColours.end(), stringValue) == allowedColours.end()) return 0;
        }
        if (field == "pid") {
          if (stringValue.size() != 9) return 0;
          if (stringValue.find_first_not_of("0123456789") != stringValue.npos) return 0;
        }
      }
      return 1;
    };

    auto isPassportValid = [&](const auto& passport) {
      if (areFieldsValid(passport) == 1 && areValuesValid(passport) == 1) return 1;
      else return 0;
    };

    return std::transform_reduce(passports.begin(), passports.end(), 0, std::plus<int>(), isPassportValid);
  }

  Solution() {
    std::sort(requiredFields.begin(), requiredFields.end());
  }

 private:
  std::vector<std::string> requiredFields{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};

  std::vector<std::vector<std::string>> parseInput(const std::string& path) {
    std::vector<std::vector<std::string>> passports;
    std::fstream input(path, std::ios_base::in);
    std::string line;
    std::vector<std::string> passport;
    while (std::getline(input, line)) {
      if (line.empty()) {
        passports.push_back(passport);
        passport.clear();
        continue;
      }

      std::stringstream ss(line);
      std::string token;
      while (ss >> token) passport.push_back(token);
    }
    passports.push_back(passport);
    return passports;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
