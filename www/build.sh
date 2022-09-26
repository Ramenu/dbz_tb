#!/bin/bash

cd ../tb_parser && wasm-pack build
cd ../www && npm start