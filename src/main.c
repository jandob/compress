#include <stdio.h>
#include <stdint.h>
#include "rust.h"

void adc_callback(const uint16_t* const values, size_t len) {
    // compress and store
    store(values, len);
}

void decompress(const uint16_t* const values, uint16_t* out, size_t len) {
    for (size_t i = 0; i < len; i++) {
        out[i] = values[i];
    }
}


