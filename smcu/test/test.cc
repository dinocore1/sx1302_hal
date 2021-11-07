#include <gtest/gtest.h>

extern "C" {
  #include <smcu.h>
}

TEST(smcu_emu, init)
{
  struct SMCU* smcu;
  ASSERT_EQ(SMCU_OK, smcu_init(&smcu));

}

