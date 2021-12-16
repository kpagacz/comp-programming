#include<iostream>
#include<string>
#include<bitset>
#include<algorithm>
#include<vector>
#include<numeric>

struct Packet {
  uint64_t version, id, number, length_type;
  std::vector<Packet *> subpackets;
  Packet() = default;
  uint64_t sum_versions() {
    uint64_t answer = version;
    std::for_each(subpackets.begin(), subpackets.end(), [&](auto p) { answer += p->sum_versions(); });
    return answer;
  }
};

Packet* parse_packet(std::string& in) {
  Packet *p = new Packet();
  p->version = std::stoull(in.substr(0, 3), 0, 2);
  in.erase(0, 3);
  p->id = std::stoull(in.substr(0, 3), 0, 2);
  in.erase(0, 3);

  if (p->id == 4) {
    char leading_char;
    std::string number;
    do
    {
      std::string batch = in.substr(0, 5);
      leading_char = batch[0];
      in.erase(0, 5);
      number += batch.substr(1, 4);
    } while (leading_char == '1');
    p->number = std::stoull(number, 0, 2);
  } else {
    std::string length_type = in.substr(0, 1);
    in.erase(0, 1);
    p->length_type = std::stoull(length_type, 0, 2);
    if (p->length_type == 0) {
      uint64_t subpackets_length = std::stoull(in.substr(0, 15), 0, 2);
      in.erase(0, 15);
      std::string subpackets_bits = in.substr(0, subpackets_length);
      in.erase(0, subpackets_length);
      while(!subpackets_bits.empty())
        p->subpackets.push_back(parse_packet(subpackets_bits));
    }
    else {
      uint64_t subpackets_number = std::stoull(in.substr(0, 11), 0, 2);
      in.erase(0, 11);
      for (uint64_t i = 0; i < subpackets_number; i++)
        p->subpackets.push_back(parse_packet(in));
    }
  }

  return p;
}

std::string
hexToBinary(const std::string &binary)
{
  std::string answer;
  for(const auto& c : binary) {
    int8_t n;
    if (c >= '0' && c <= '9') {
      n = c - '0';
    } else {
      n = 10 + c - 'A';
    }
    for (auto j = 3; j >= 0; j--)
      answer += (n & (1U << j)) ? '1' : '0';
  }
  return answer;
}

int main(int argc, char* argv[]) {
  // read input
  std::string in;
  while(std::cin >> in) {
    in = hexToBinary(in);
    Packet* p = parse_packet(in);
    std::cout << "Sum of versions: " << p->sum_versions() << '\n';
  }
}
