CC=g++
CFLAGS=-std=c++2a
OUTPUT=solution
SRC=solution.cpp

run: compile
	./$(OUTPUT)

compile: $(SRC)
	$(CC) $(SRC) -o $(OUTPUT) $(CFLAGS)

clean:
	rm -f $(OUTPUT)
