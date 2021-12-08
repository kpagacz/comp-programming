#include<string>
#include<iostream>
#include<vector>
#include<algorithm>
#include<unordered_map>
#include<sstream>
#include<numeric>

#define DEBUG 0

std::vector<std::string> read_digits(int digits_number, std::stringstream& input) {
  std::vector<std::string> digits(digits_number);
  for(auto& digit : digits) {
    input >> digit;
    std::sort(digit.begin(), digit.end());
  }
  return digits;
}

std::string map_digit(std::string digit, std::string one, std::string four) {
  std::string answer;
  int common_with_one = std::count_if(digit.begin(), digit.end(), [&](char c)
                                      { return one.find(c) != std::string::npos; });
  int common_with_four = std::count_if(digit.begin(), digit.end(), [&](char c)
  { return four.find(c) != std::string::npos; });
  switch(digit.size()) {
    case 2:
      answer = "1";
      break;
    case 3:
      answer = "7";
      break;
    case 4:
      answer = "4";
      break;
    case 5:
      if (common_with_one == 1 && common_with_four == 2)
        answer = "2";
      if (common_with_one == 2 && common_with_four == 3)
        answer = "3";
      if (common_with_one == 1 && common_with_four == 3)
        answer = "5";
      break;
    case 6:
      if (common_with_one == 2 && common_with_four == 3)
        answer = "0";
      if (common_with_one == 1 && common_with_four == 3)
        answer = "6";
      if (common_with_one == 2 && common_with_four == 4)
        answer = "9";
      break;
    case 7:
      answer = "8";
      break;
  }
  return answer;
}

int testCase(std::stringstream& input) {
  int answer = 0;
  std::vector<std::string> encoded = read_digits(10, input);
  std::vector<std::string> display = read_digits(4, input);

  std::string one = *std::find_if(encoded.begin(), encoded.end(), [&](std::string digit)
                                 { return digit.size() == 2; });
  std::string four = *std::find_if(encoded.begin(), encoded.end(), [&](std::string digit)
                                  { return digit.size() == 4; });

  auto displayed_number = std::accumulate(display.begin(), display.end(), std::string(), [&](std::string a, std::string b)
                                          { return a + map_digit(b, one, four); });

  if (DEBUG)
    std::cout << displayed_number << '\n';
  return std::stoi(displayed_number, 0, 10);
}


int main() {
  std::string line;
  int answer = 0;
  while(std::getline(std::cin, line)) {
    line.erase(std::find(line.begin(), line.end(), '|'));
    std::stringstream input(line);
    answer += testCase(input);
  }

  std::cout << "Sum of all the displayed numbers: " << answer << '\n';
}
