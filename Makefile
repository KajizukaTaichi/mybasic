OUTPUT = test
SRC = test.asm
OBJ = test.o

all: $(OUTPUT)

$(SRC):
	cargo run > $(SRC)

$(OBJ): $(SRC)
	nasm $(SRC) -f macho64 -o $(OBJ)

$(OUTPUT): $(OBJ)
	clang $(OBJ) -o $(OUTPUT)

clean:
	rm -f $(SRC) $(OBJ) $(OUTPUT)

# Phony targets (these are not filenames)
.PHONY: all clean
