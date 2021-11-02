#include <libsmcu_emu.h>

#include <ed25519.h>

#include <string.h>


static
int write_i32_msb(uint8_t* buf, uint32_t v) {
  buf[0] = (v >> 24) & 0xff;
  buf[1] = (v >> 16) & 0xff;
  buf[2] = (v >>  8) & 0xff;
  buf[3] = (v >>  0) & 0xff;
  return 4;
}

int smcu_emu_init(struct smcu_emu* p, uint8_t* key_pub, uint8_t* key_priv)
{
  memcpy(p->key_pub, key_pub, 32);
  memcpy(p->key_priv, key_priv, 64);
  return 0;
}

int smcu_emu_sign(struct smcu_emu* p, uint8_t* sig, struct lgw_pkt_rx_s* pkt)
{
  ed25519_sign(sig, data, data_len, p->key_pub, p->key_priv);
  return 0;
}