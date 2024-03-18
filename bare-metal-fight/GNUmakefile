ASSEMBLER?=riscv64-linux-gnu-as
ASSEMBLER_FLAGS?=-march=rv32g -mabi=ilp32d
LINKER?=riscv64-linux-gnu-ld
LINKER_FLAGS?=-T

all: bin/fight.elf

bin/%.elf: src/%.ld
	@mkdir -p bin
	$(LINKER) $(LINKER_FLAGS) $^ -o $@

bin/fight.elf: src/fight.ld obj/fight.o

obj/%.o: src/%.s
	@mkdir -p obj
	$(ASSEMBLER) $(ASSEMBLER_FLAGS) -o $@ $<

clean:
	rm -rf bin obj

.PHONY: all clean
.SECONDARY: # do not kill "intermediate products"
MAKEFLAGS += --no-builtin-rules
