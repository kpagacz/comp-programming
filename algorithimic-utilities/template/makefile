CC=g++-12
CFLAGS=-std=c++2b
OUTPUT=solution.exe
SRC=solution.cpp

run: compile
	./$(OUTPUT)

compile: $(SRC)
	$(CC) $(SRC) -o $(OUTPUT) $(CFLAGS)

clean:
	rm -f $(OUTPUT)
