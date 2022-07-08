WebAssembly.instantiateStreaming(fetch('main.wasm'), {}).then(result => {
    console.log(result.instance.exports.hello_sum(500, 400))
});
