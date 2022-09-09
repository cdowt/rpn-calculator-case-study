.POSIX:

default: bin/rpn-calculator

CFLAGS += -std=c11 -pedantic
CFLAGS += -Wall -Wextra

CC = arm-none-eabi-gcc
AR = arm-none-eabi-ar
AS = arm-none-eabi-as

include stm32f40_41xxx.mk

CFLAGS += -I inc

SRC = src/io.c src/main.c
OBJ = $(SRC:.c=.o)

bin/rpn-calculator: $(DEVICE_LIBS) $(OBJ)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $(OBJ) $(DEVICE_LIBS)

compile_commands.json: Makefile
	$(MAKE) clean
	bear -- $(MAKE)

clean:
	rm -f bin/*
	find . -name '*.o' | xargs -n 1 rm -f

.PHONY: clean default
