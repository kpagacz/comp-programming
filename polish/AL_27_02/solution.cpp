// link to the problem https://pl.spoj.com/problems/AL_27_02/
#include <string>
#include <iostream>
#include <sstream>

int main() {
	std::stringstream line_stream;
	std::string line;
	std::string word;
	while (std::getline(std::cin, line)) {
		line_stream = std::stringstream(line);
		while (line_stream >> word) {
			std::cout << word.front();
			if (word.back() == '.') std::cout << " ";
		}

		std::cout << "\n";
	}
	
}