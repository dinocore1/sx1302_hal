### get external defined data

### constant symbols

ARCH ?=
CROSS_COMPILE ?=
CC := $(CROSS_COMPILE)gcc
AR := $(CROSS_COMPILE)ar

CFLAGS := -O2 -Wall -Wextra -std=c99 -Iinc -I. -Wno-strict-aliasing

OBJDIR = obj
INCLUDES = $(wildcard inc/*.h)

### linking options

### general build targets

all: libed25519.a

clean:
	rm -f libed25519.a
	rm -f $(OBJDIR)/*.o

### library module target

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(OBJDIR)/%.o: src/%.c $(INCLUDES) | $(OBJDIR)
	$(CC) -c $(CFLAGS) $< -o $@

### static library

libed25519.a:  $(OBJDIR)/ge.o $(OBJDIR)/sc.o $(OBJDIR)/fe.o $(OBJDIR)/sha512.o $(OBJDIR)/sign.o $(OBJDIR)/verify.o $(OBJDIR)/keypair.o
	$(AR) rcs $@ $^

### test programs

### EOF
