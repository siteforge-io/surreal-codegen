#!/usr/bin/env node
import surreal_codegen from './pkg/surreal_codegen.js';

async function main() {
    await surreal_codegen.default(); // This loads the WASM module
    surreal_codegen.main();
}

main().catch(console.error);
