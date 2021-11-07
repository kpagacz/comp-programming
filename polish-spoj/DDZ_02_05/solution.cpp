// link to the problem https://pl.spoj.com/problems/DDZ_02_05/
#define _CRT_SECURE_NO_WARNINGS
#include<cstdio>
#include<cstring>
#include<vector>
#include<algorithm>

struct piece {
	char name, posx;
	int posy;
	piece(char name, char posx, int posy) {
		this->name = name;
		this->posx = posx;
		this->posy = posy;
	}

	void print() {
		printf("%c %c%d\n", name, posx, posy);
	}
};

int main() {
	int black_pieces_count, white_pieces_count;
	scanf("%d %d\n", &black_pieces_count, &white_pieces_count);

	char black_pieces[64];
	std::memset(black_pieces, '0', 64);

	char white_pieces[64];
	std::memset(white_pieces, '0', 64);

	std::vector<piece> white_pieces_vector;
	std::vector<piece> black_pieces_vector;

	char piece_name, posx;
	int posy;
	for (int i = 0; i < black_pieces_count; ++i) {
		scanf(" %c %c%d\n", &piece_name, &posx, &posy);
		black_pieces[posx - 'a' + 8 * (posy - 1)] = piece_name;
		black_pieces_vector.push_back(piece(piece_name, posx, posy));
	}

	for (int i = 0; i < white_pieces_count; ++i) {
		scanf(" %c %c%d\n", &piece_name, &posx, &posy);
		white_pieces[posx - 'a' + 8 * (posy - 1)] = piece_name;
		white_pieces_vector.push_back(piece(piece_name, posx, posy));
	}

	// sorting
	auto comparator = [](const piece a, const piece b) { return a.name == b.name ? a.posx < b.posx : a.name < b.name; };
	std::sort(std::begin(white_pieces_vector), std::end(white_pieces_vector), comparator);
	std::sort(std::begin(black_pieces_vector), std::end(black_pieces_vector), comparator);

	// printing
	for (int i = 7; i >= 0; --i) {
		for (int j = 0; j < 8; ++j) {
			printf("%c", white_pieces[i * 8 + j]);
		}
		printf("\n");
	}
	for (piece p : white_pieces_vector) p.print();

	printf("\n");
	for (int i = 7; i >= 0; --i) {
		for (int j = 0; j < 8; ++j) {
			printf("%c", black_pieces[i * 8 + j]);
		}
		printf("\n");
	}
	for (piece p : black_pieces_vector) p.print();
}