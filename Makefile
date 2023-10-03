CC = g++
CFLAGS = -g -Wall -c
OBJS = tokenize.o
.PHONY = clean

test: main.cpp tokenize.o
	$(CC) $(OBJS) main.cpp -o test

tokenize.o: tokenize.cpp
	$(CC) $(CFLAGS) tokenize.cpp -o tokenize.o

clean:
	rm tokenize.o test
