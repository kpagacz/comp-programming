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
  int score;

  BingoBlock(int size) : size(size) {
    numbers = new int[size * size];
    marked = new bool[size * size];
    std::memset(numbers, 0, sizeof(int) * size * size);
    std::memset(marked, false, sizeof(bool) * size * size);
  }

  bool mark_number(int number) {
    for (int i = 0; i < size * size; i++) {
      if (numbers[i] == number) {
        marked[i] = true;
        if(check_bingo(i)) {
          score = sum_unmarked() * number;
          return true;
        }
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
  int highest_moves = 0;
  int board_id = 0;
  for (int i = 0; i < blocks.size(); i++) {
    for (int j = 0; j < bingo_numbers.size();j++) {
      bool bingo = blocks[i]->mark_number(bingo_numbers[j]);
      if (bingo) {
        if (j > highest_moves) {
          highest_moves = j;
          board_id = i;
        }
        break;
      }
    }
  }
  std::cout << "board with longest moves: " << board_id << '\n';
  std::cout << "board score: " << blocks[board_id]->score << '\n';
}
