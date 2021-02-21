# makefile for hello world

GCC=rustc
EXT=rs
TARGET=hello

all: 
	$(GCC) $(TARGET).$(EXT)

clean:
	rm -f $(TARGET)
