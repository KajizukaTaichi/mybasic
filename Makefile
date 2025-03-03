OUTPUT = test
SRC = test.asm
OBJ = test.o

all: $(OUTPUT)

$(SRC):
	cargo run > $(SRC)

$(OBJ): $(SRC)
	nasm -f macho32 -o $(OBJ) $(SRC)

$(OUTPUT): $(OBJ)
	clang $(OBJ) -o $(OUTPUT)

clean:
	rm -f $(OBJ) $(OUTPUT)

# Phony targets (these are not filenames)
.PHONY: all clean
