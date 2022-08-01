import init, { Simulation } from "../engine/pkg/simple_ball_engine.js";

console.log("Running");

async function app() {
  console.log("starting init ...");
  await init();
  console.log("init done");

  const canvas = document.getElementById("canvas");
  const context = canvas.getContext("2d");

  const maxX = canvas.width - 1;
  const maxY = canvas.height - 1;

  const minDimension = Math.min(maxX, maxY);
  const ballRadius = 0.05 * minDimension;
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

  // Simulation area is a 100x100 box, which we map to our maxX, maxY
  // area. We place a single ball.
  const sim = new Simulation(50.0, 50.0, ballRadius / minDimension);
  const simWidth = 100.0;
  const simHeight = 100.0;
  const scaleX = maxX / simWidth;
  const scaleY = maxY / simHeight;
  function updateBall(sim_x, sim_y) {
    ball.x = clampX(sim_x * scaleX);
    ball.y = clampY(maxY - sim_y * scaleY); // y is inverted in sim vs display
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
      lastUpdate = 0;
    } else {
      const forceScale = 0.1;
      sim.set_force(
        (2.0 * Math.random() - 1.0) * forceScale,
        (2.0 * Math.random() - 1.0) * forceScale
      );

      const elapsed = timestamp - start;
      const elapsedSinceLastUpdate = elapsed - lastUpdate;
      sim.update(elapsedSinceLastUpdate, updateBall);
      lastUpdate = elapsed;
    }
    draw();

    window.requestAnimationFrame(animate);
  }

  window.requestAnimationFrame(animate);
}

await app();
