ls -al target/wa*/release/*.wasm
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/echo2.wasm \
    --id 1 \
    -- \
    echo2\
    --to moonlight

