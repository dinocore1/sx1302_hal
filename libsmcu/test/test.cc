#include <gtest/gtest.h>

extern "C" {
  #include <smcu.h>
}

TEST(smcu_emu, init)
{
  struct SMCU* smcu;
  ASSERT_EQ(SMCU_OK, smcu_init(&smcu));

  smcu_free(smcu);
}

TEST(smcu_emu, sign)
{
  struct SMCU* smcu;
  smcu_init(&smcu);

  signature_t sig;
  LoraPacket pkt;
  pkt.data_len = 0;

  smcu_sign(smcu, &sig, &pkt);

  smcu_free(smcu);
}

