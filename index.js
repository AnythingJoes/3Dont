let memory;
let canvas = document.querySelector("#app")
let context = canvas.getContext("2d")

WebAssembly.instantiateStreaming(fetch('main.wasm'), {
    env: {
        draw_triangle: vertPtr =>  {
            let vertices = new Float32Array(memory.buffer.slice(vertPtr, vertPtr + 9 * 4))
            context.beginPath()
            context.fillStyle = `rgb(${Math.floor(255 * 1.0)}, ${Math.floor(255 * 0.5)}, ${Math.floor(255 * 0.2)}, 255)`
            context.strokeStyle = `rgb(${Math.floor(255 * 1.0)}, ${Math.floor(255 * 0.5)}, ${Math.floor(255 * 0.2)}, 255)`

            let [x, y] = vertices
            x = ((x + 1) / 2) * canvas.width
            y = (canvas.height - ((y + 1) / 2) * canvas.height)


            context.moveTo(x, y)

            for (let i = 3; i < vertices.length; i += 3) {
                let x1 = vertices[i]
                let y1 = vertices[i + 1]
                x1 = ((x1 + 1) / 2) * canvas.width
                y1 = (canvas.height - ((y1 + 1) / 2) * canvas.height)
                context.lineTo(x1, y1)
            }
            context.lineTo(x, y)

            context.fill()
            context.stroke()
        },
    }
}).then(result => {
    memory = result.instance.exports.memory
    canvas.style.backgroundColor = `rgb(${Math.floor(255 * 0.2)}, ${Math.floor(255 * 0.3)}, ${Math.floor(255 * 0.3)}, 255)`
    context.clearRect(0, 0, canvas.width, canvas.height)
    context.lineWidth = 2
    result.instance.exports.render()
});
