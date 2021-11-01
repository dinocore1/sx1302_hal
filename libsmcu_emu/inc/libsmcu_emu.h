#ifndef _SMCU_EMU_H
#define _SMCU_EMU_H

#include <stdint.h>     /* C99 types*/
#include <stddef.h>

struct smcu_emu {
  uint8_t key_pub[32];
  uint8_t key_priv[32];
};

int smcu_emu_init(struct smcu_emu* p, uint8_t* key_pub, uint8_t* key_priv);

int smcu_emu_sign(struct smcu_emu* p, uint8_t* sig, uint8_t* data, size_t data_len);

#endif // _SMCU_EMU_H