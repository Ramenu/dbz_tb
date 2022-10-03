#!/bin/bash

cd ../tb_parser && wasm-pack build --target web --out-dir ../tb_web/src/pkg
cd ../tb_web && npm install ./src/pkg && npm start