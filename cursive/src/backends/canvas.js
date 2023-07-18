const fontWidth = 12;
const fontHeight = fontWidth * 2;

export function paint(buffer) {
    // console.log("buffer", JSON.stringify(buffer));
    const data = buffer.data;
    // console.log("data", JSON.stringify(data));
    const canvas = document.getElementById('cursive-wasm-canvas');
    const context = canvas.getContext('2d');
    const backBuffer = new Map();
    const frontBuffer = new Map();
    for (let x = 0; x < 1000; x++) {
        for (let y = 0; y < 1000; y++) {
            let { text, color } = data[1000 * y + x];
            if (text != ' ') {
                const buffer = frontBuffer.get(color.front) || [];
                buffer.push({ x, y, text });
                frontBuffer.set(color.front, buffer);
            }
            const buffer = backBuffer.get(color.back) || [];
            buffer.push({ x, y });
            backBuffer.set(color.back, buffer);
        }
    }
    console.log(backBuffer.size, frontBuffer.size);
    backBuffer.forEach((buffer, back) => {
        context.fillStyle = back;
        buffer.forEach(value => {
            context.fillRect(value.x * fontWidth, value.y * fontHeight, fontWidth, fontHeight);
        });
    });
    frontBuffer.forEach((buffer, front) => {
        context.fillStyle = front;
        buffer.forEach(value => {
            context.fillText(value.letter, value.x * fontWidth, value.y * fontHeight);
        });
    });
}
