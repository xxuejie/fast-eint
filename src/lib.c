#include <stdint.h>
#include <string.h>
#include <stdio.h>

void narrowing_right_shift_512_c_impl(const uint64_t *src, uint64_t *dst, uint32_t shift, uint64_t len) {
  shift = shift & 511;
  uint64_t qword_shift = shift % 64;
  uint64_t start = shift / 64;

  for (uint64_t i = 0; i < len; i++) {
    const uint64_t *csrc = &src[i * 8];
    uint64_t *cdst = &dst[i * 4];

    uint64_t values[5] = {0, 0, 0, 0, 0};

    uint64_t len = (8 - start) > 5 ? 5 : (8 - start);
    memcpy(values, &csrc[start], len * 8);

    if (qword_shift > 0) {
      uint64_t shift1 = values[1] << (64 - qword_shift);
      uint64_t shift2 = values[2] << (64 - qword_shift);
      uint64_t shift3 = values[3] << (64 - qword_shift);
      uint64_t shift4 = values[4] << (64 - qword_shift);

      values[0] = (values[0] >> qword_shift) | shift1;
      values[1] = (values[1] >> qword_shift) | shift2;
      values[2] = (values[2] >> qword_shift) | shift3;
      values[3] = (values[3] >> qword_shift) | shift4;
    }

    memcpy(cdst, values, 4 * 8);
  }
}
