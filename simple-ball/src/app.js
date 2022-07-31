// import init, { greet } from "../engine/pkg/simple_ball_engine.js";

console.log("Running");

// async function app() {
//   console.log("init");
//   await init();

//   console.log("greet");

//   greet("hello!");
// }
// await app();

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

const maxX = canvas.width;
const maxY = canvas.height;

const ballRadius = 0.05 * Math.min(maxX, maxY);
const ball = {
  x: 0.5 * maxX,
  y: 0.5 * maxY,
};

function clampX(x) {
  return Math.min(Math.max(0, x), maxX - ballRadius);
}

function clampY(y) {
  return Math.min(Math.max(0, y), maxY - ballRadius);
}

function updateBall(x, y) {
  ball.x = clampX(x);
  ball.y = clampY(y);
}

function update(elapsedSinceLastUpdate, updateFn) {
  const speed = 0.3; // pixels per millisecond
  const distance = speed * elapsedSinceLastUpdate;
  const angle = 2 * Math.PI * Math.random();
  const xChange = Math.cos(angle) * distance;
  const yChange = Math.sin(angle) * distance;

  updateFn(ball.x + xChange, ball.y + yChange);
}

function draw() {
  context.clearRect(0, 0, maxX, maxY);

  context.beginPath();
  context.arc(ball.x, ball.y, ballRadius, 0, 2 * Math.PI);
  context.fill();
}

var start = undefined;
var lastUpdate = undefined;
function animate(timestamp) {
  if (start === undefined) {
    start = timestamp;
    lastUpdate = start;
  }
  const elapsed = timestamp - start;
  const elapsedSinceLastUpdate = elapsed - lastUpdate;
  update(elapsedSinceLastUpdate, updateBall);
  lastUpdate = elapsed;
  draw();

  window.requestAnimationFrame(animate);
}

window.requestAnimationFrame(animate);
