#!/bin/bash

bindgen --use-core --merge-extern-blocks -o src/bindings.rs perfect6502/perfect6502.h