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
  pkt.data[0] = 'a';
  pkt.data_len = 1;
  pkt.freq_hz = 902489000;
  pkt.datarate = 6;
  pkt.bandwidth = 0x04;

  EXPECT_EQ(SMCU_OK, smcu_sign(smcu, &sig, &pkt));

  smcu_free(smcu);
}

