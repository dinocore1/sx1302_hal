#include <libsmcu_emu.h>

#include <ed25519.h>

#include <string.h>

int smcu_emu_init(struct smcu_emu* p, uint8_t* key_pub, uint8_t* key_priv)
{
  memcpy(p->key_pub, key_pub, 32);
  memcpy(p->key_priv, key_priv, 32);
  return 0;
}

int smcu_emu_sign(struct smcu_emu* p, uint8_t* sig, uint8_t* data, size_t data_len)
{
  ed25519_sign(sig, data, data_len, p->key_pub, p->key_priv);
  return 0;
}