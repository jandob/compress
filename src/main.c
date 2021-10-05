#include <stdio.h>
#include <stdint.h>
#include "rust.h"

void adc_callback(const uint16_t* const values, size_t len) {
    // compress and store
    store((uint8_t*) values, len*2);
}

// decompress from buffer
void decompress(const uint8_t* const values, uint16_t* out, size_t len_values, size_t len_out) {
    for (size_t i = 0; i < len_out; i++) {
        out[i] = values[i*2] | values[i*2+1] << 8;
    }
}


