# makefile for hello world

GCC=rustc
EXT=rs
TARGET=out

all: 
	$(GCC) -o hello hello.$(EXT)
	$(GCC) -o debugs debugs.$(EXT)
	$(GCC) -o basic_structures basic_structures.$(EXT)
	$(GCC) -o vector_examples vector_examples.$(EXT)
	$(GCC) -o $(TARGET) main.$(EXT)

clean:
	rm -f hello 
	rm -f debugs
	rm -f basic_structures
	rm -f vector_examples
	rm -f $(TARGET)
