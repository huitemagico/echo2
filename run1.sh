ls -al target/wa*/rel*logs/*.wasm
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \
    --id 1 \
    -- \
    echo2\
    --to alejandrito

