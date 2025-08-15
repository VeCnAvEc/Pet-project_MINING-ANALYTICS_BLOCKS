#!/bin/bash

# Устанавливаем уровень логирования
export RUST_LOG=info

# Компилируем и запускаем reader
cargo run --bin main_reader 