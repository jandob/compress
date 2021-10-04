#include <stdio.h>
#include <stdint.h>
#include "rust.h"

adc_callback(const uint16_t* const values, size_t len) {
    // compress and store
    store(values, len);
}

decompress(const uint16_t* const values, uint16_t* out, size_t len) {
    // compress and store
    for (size_t i = 0; i < len; i++) {
        out[i] = values[i]
    }
}


