#include <gtest/gtest.h>

extern "C" {
  #include <libsmcu_emu.h>
  #include <ed25519.h>
  #include <sha-256.h>
}



TEST(smcu_emu, sign)
{
  struct smcu_emu smcu;
  uint8_t pub[32];
  uint8_t priv[32];
  uint8_t seed[32];
  uint8_t sig[64];

  calc_sha_256(seed, "hello_world", 11);
  ed25519_create_keypair(pub, priv, seed);

  smcu_emu_init(&smcu, pub, priv);

  smcu_emu_sign(&smcu, sig, (uint8_t*)"awesome", 6);

  EXPECT_EQ(1, ed25519_verify(sig, (uint8_t*)"awesome", 6, pub));
  EXPECT_EQ(0, ed25519_verify(sig, (uint8_t*)"awesZme", 6, pub));


}