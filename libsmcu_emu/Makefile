### get external defined data

### constant symbols

ARCH ?=
CROSS_COMPILE ?=
CC := $(CROSS_COMPILE)gcc
AR := $(CROSS_COMPILE)ar

CFLAGS := -O2 -Wall -Wextra -std=c99 -Iinc -I. -Wno-strict-aliasing

OBJDIR = obj
INCLUDES = $(wildcard inc/*.h) $(wildcard ../libed25519/inc/*.h)

### linking options

### general build targets

all: libsmcu_emu.a

clean:
	rm -f libsmcu_emu.a
	rm -f $(OBJDIR)/*.o

### library module target

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(OBJDIR)/%.o: src/%.c $(INCLUDES) | $(OBJDIR)
	$(CC) -c $(CFLAGS) $< -o $@

### static library

libsmcu_emu.a:  $(OBJDIR)/smcu_emu.o
	$(AR) rcs $@ $^

### test programs



### EOF
