### get external defined data

### constant symbols

ARCH ?=
CROSS_COMPILE ?=
CC := $(CROSS_COMPILE)gcc
CXX := $(CROSS_COMPILE)g++
AR := $(CROSS_COMPILE)ar

CFLAGS := -O2 -Wall -Wextra -std=c99 -Iinc -I. -I../libed25519/inc -Wno-strict-aliasing

OBJDIR = obj
INCLUDES = $(wildcard inc/*.h) $(wildcard ../libed25519/inc/*.h)

### linking options

LIBS := -lsmcu_emu -led25519

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

$(OBJDIR)/%.o: src/%.cc $(INCLUDES) | $(OBJDIR)
	$(CXX) -c $(CXXFLAGS) $< -o $@

### static library

libsmcu_emu.a:  $(OBJDIR)/smcu_emu.o
	$(AR) rcs $@ $^

### test programs

test_units: tests/test_units.cc libsmcu_emu.a
	$(CXX) -L. -L../libed25519 -Iinc -I. -I../libed25519/inc -g -O0  $< -o $@ $(LIBS) -lgtest_main -lgtest -lpthread


### EOF
