#include<string>
#include<iostream>
#include<vector>
#include<algorithm>
#include<unordered_map>
#include<sstream>

std::vector<std::string> readDigits(int digits_number, std::stringstream& input) {
  std::vector<std::string> digits(digits_number);
  for(auto& digit : digits) {
    input >> digit;
    std::sort(digit.begin(), digit.end());
  }
  return digits;
}

std::unordered_map<std::string, int> decodeDigits(std::vector<std::string>& digits) {
  std::unordered_map<std::string, int> decoded_digits;

  // find 1, 4, 7 ,8
  std::string one = *std::find_if(digits.begin(), digits.end(), [](std::string dig)
                                 { return dig.size() == 2; });
  decoded_digits.insert({one, 1});

  std::string four = *std::find_if(digits.begin(), digits.end(), [](std::string dig)
                                 { return dig.size() == 4; });
  decoded_digits.insert({four, 4});

  std::string seven = *std::find_if(digits.begin(), digits.end(), [](std::string dig)
                                 { return dig.size() == 3; });
  decoded_digits.insert({seven, 7});

  std::string eight = *std::find_if(digits.begin(), digits.end(), [](std::string dig)
                                  { return dig.size() == 7; });
  decoded_digits.insert({eight, 8});

  // find rest
  auto contains_one = [&](std::string digit) {
    return digit.find(one[0]) != std::string::npos && digit.find(one[1]) != std::string::npos;
  };

  std::string three = *std::find_if(digits.begin(), digits.end(), [&](std::string digit)
                                    { return contains_one(digit) && digit.size() == 5; });
  decoded_digits.insert({three, 3});

  auto contains_four = [&](std::string digit) {
    for(const auto& c : four) {
      if (digit.find(c) == std::string::npos)
        return false;
    }
    return true;
  };
  std::string nine = *std::find_if(digits.begin(), digits.end(), [&](std::string digit)
                                   { return contains_four(digit) && digit.size() == 6;  });
  decoded_digits.insert({nine, 9});

  std::string zero = *std::find_if(digits.begin(), digits.end(), [&](std::string digit)
                                   { return digit != nine && contains_one(digit); });
  decoded_digits.insert({zero, 0});

  std::string six = *std::find_if(digits.begin(), digits.end(), [&](std::string digit)
                                  { return digit.size() == 6 && digit != nine && digit != zero; });
  decoded_digits.insert({six, 6});

  auto is_two = [&](std::string digit) {
    std::string result(digit);
    result += four;
    std::sort(result.begin(), result.end());
    result.erase(std::unique(result.begin(), result.end()), result.end());
    std::sort(result.begin(), result.end());
    return result == eight && digit.size() == 5;
  };

  std::string two = *std::find_if(digits.begin(), digits.end(), [&](std::string digit)
                                 { return is_two(digit); });
  decoded_digits.insert({two, 2});

  std::string five = *std::find_if(digits.begin(), digits.end(), [&](std::string digit)
                                   { return digit.size() == 5 && digit != two && digit != three; });
  decoded_digits.insert({five, 5});

  return decoded_digits;
}

int testCase(std::stringstream& input) {
  int answer = 0;
  std::vector<std::string> encoded = readDigits(10, input);
  // input.ignore(3); // because there is | in input
  std::vector<std::string> display = readDigits(4, input);

  auto decoded = decodeDigits(encoded);
  // for(const auto& p : decoded) {
  //   std::cout << "Encoding: " << p.first << "  digit: " << p.second << '\n';
  // }

  std::vector<int> counted_digits({1, 4, 7, 8});
  answer += std::count_if(display.begin(), display.end(), [&](std::string digit)
                          { return decoded[digit] == 1 || decoded[digit] == 4 || decoded[digit] == 7 || decoded[digit] == 8; });

  return answer;
}


int main() {
  std::string line;
  int answer = 0;
  while(std::getline(std::cin, line)) {
    line.erase(std::find(line.begin(), line.end(), '|'));
    std::stringstream input(line);
    answer += testCase(input);
  }

  std::cout << "Total 1,4,7,8: " << answer << '\n';
}
