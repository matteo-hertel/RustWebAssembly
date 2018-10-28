import {memory} from '../crate/pkg/rust_webpack_bg';
import {Image} from '../crate/pkg/rust_webpack';

console.log(memory);

const image = Image.new();
const pixelPointer = image.pixels_ptr();
const pixels = new Uint8Array(memory.buffer, pixelPointer, 6);

function drawPixels(canvas, x, y, color) {
  const ctx = canvas.getContext('2d');
  ctx.fillStyle = `#${numToHex(color[0])}${numToHex(color[1])}${numToHex(
    color[2],
  )}`;
  ctx.fillRect(x, y, 100, 100);
}

function numToHex(value) {
  const hex = value.toString(16);
  return hex.length === 1 ? `0${hex}` : hex;
}
const canvas = document.createElement('canvas');
document.body.appendChild(canvas);
drawPixels(canvas, 0, 0, pixels.slice(0, 3));
drawPixels(canvas, 100, 0, pixels.slice(3, 6));
