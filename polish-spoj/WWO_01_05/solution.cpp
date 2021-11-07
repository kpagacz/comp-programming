// link to the problem https://pl.spoj.com/problems/WWO_01_05/
#include <iostream>
#include <string>
#include <bitset>

int main() {
	std::string digit1, digit2, digit3, digit4;
	std::bitset<8> digit1_bits;
	std::bitset<8> digit2_bits;
	std::bitset<8> digit3_bits;
	std::bitset<8> digit4_bits;

	std::cin >> digit1 >> digit2 >> digit3 >> digit4;
	digit1_bits = std::bitset<8>(digit1);
	digit2_bits = std::bitset<8>(digit2);
	digit3_bits = std::bitset<8>(digit3);
	digit4_bits = std::bitset<8>(digit4);

	std::cout << digit1_bits.to_ulong() << digit2_bits.to_ulong() << ":" << digit3_bits.to_ulong() << digit4_bits.to_ulong();
}