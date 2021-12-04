#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<cstring>

std::vector<int> parse_first_line() {
  std::vector<int> numbers;
  std::string line;
  std::getline(std::cin, line);

  size_t start = 0, end = 0;
  std::string delimiter = ",";
  std::string token;
  while((end = line.find(delimiter, start)) != std::string::npos) {
    token = line.substr(start, end);
    numbers.push_back(std::stoi(token, 0, 10));
    start = end + delimiter.size();
  }
  token = line.substr(start, end);
  numbers.push_back(std::stoi(token, 0, 10));
  start = end + delimiter.size();

  return numbers;
};

struct BingoBlock {
  int *numbers;
  bool *marked;
  int size;

  BingoBlock(int size) : size(size) {
    numbers = new int[size * size];
    marked = new bool[size * size];
    std::memset(numbers, 0, sizeof(int) * size * size);
    std::memset(marked, false, sizeof(bool) * size * size);
  }

  int& operator[](int coord) {
    return numbers[coord];
  }

  bool mark_number(int number) {
    for (int i = 0; i < size * size; i++) {
      if (numbers[i] == number) {
        marked[i] = true;
        if(check_bingo(i))
          return true;
      }
    }
    return false;
  }

  bool check_bingo(int coord) {
    int row = coord / size;
    int col = coord % size;

    bool bingo = true;
    for (int i = 0; i < size; i++) {
      if (marked[row * size + i] == false) {
        bingo = false;
        break;
      }
    }

    if (bingo)
      return bingo;

    bingo = true;
    for (int i = 0; i < size; i++) {
      if(marked[i * size + col] == false) {
        bingo = false;
        break;
      }
    }
    return bingo;
  }

  void print() {
    for (int i = 0; i < size * size; i++)
      std::cout << numbers[i] << " ";
    std::cout << "\n";
  }

  int sum_unmarked() {
    int sum = 0;
    for (int i = 0; i < size * size; i++) if (!marked[i])
        sum += numbers[i];
    return sum;
  }

  ~BingoBlock() {
    delete[] numbers;
    delete[] marked;
  }
};

int main() {
  int block_size = 5;
  // read the first line
  std::vector<int> bingo_numbers = parse_first_line();

  // read the bingoblocks
  std::vector<BingoBlock*> blocks;
  int number;
  while(std::cin >> number) {
    BingoBlock* new_block = new BingoBlock(block_size);
    new_block->numbers[0] = number;
    for (int i = 1; i < block_size * block_size; i++)
      std::cin >> new_block->numbers[i];

    blocks.push_back(new_block);
  }

  // find bingo
  for(const auto& number : bingo_numbers) {
    bool bingo = false;
    for (int i = 0; i < blocks.size(); i++) {
      bingo = blocks[i]->mark_number(number);
      if (bingo) {
        std::cout << "bingo on board: " << i << '\n';
        std::cout << "unmarked sum: " << blocks[i]->sum_unmarked() << '\n';
        std::cout << "bingo number: " << number << '\n';
        std::cout << "sum * bingo number: " << number * blocks[i]->sum_unmarked() << '\n';
        break;
      }
    }
    if (bingo)
      break;
  }
}
