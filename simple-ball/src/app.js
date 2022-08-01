import init, { Simulation } from "../engine/pkg/simple_ball_engine.js";

console.log("Running");

async function app() {
  console.log("starting init ...");
  await init();
  console.log("init done");

  const deviceMotionListener = (event) => {
    const { alpha, beta, gamma } = event;
    document.getElementById("motion_alpha").innerText = alpha.toFixed(0);
    document.getElementById("motion_beta").innerText = beta.toFixed(0);
    document.getElementById("motion_gamma").innerText = gamma.toFixed(0);
    console.dir(event);
  };
  function enableDeviceMotion() {
    if (window.DeviceMotionEvent) {
      console.log("supports DeviceMotionEvent");
      if (DeviceMotionEvent.requestPermission) {
        console.log("must request permission for DeviceMotionEvent");
        DeviceMotionEvent.requestPermission().then((response) => {
          if (response == "granted") {
            window.addEventListener("devicemotion", deviceMotionListener);
          } else {
            console.log("no permission for DeviceMotionEvent");
          }
        });
      } else {
        console.log("no permission required for DeviceMotionEvent");
        window.addEventListener("devicemotion", deviceMotionListener);
      }
    } else {
      console.log("does not support DeviceMotionEvent");
    }
  }
  function enableDeviceOrientation() {
    const deviceOrientationListener = (event) => {
      const { alpha, beta, gamma } = event;
      document.getElementById("orientation_alpha").innerText = alpha.toFixed(0);
      document.getElementById("orientation_beta").innerText = beta.toFixed(0);
      document.getElementById("orientation_gamma").innerText = gamma.toFixed(0);
      console.dir(event);
    };
    if (window.DeviceOrientationEvent) {
      console.log("supports DeviceOrientationEvent");
      if (DeviceOrientationEvent.requestPermission) {
        console.log("must request permission for DeviceOrientationEvent");
        DeviceOrientationEvent.requestPermission().then((response) => {
          if (response == "granted") {
            window.addEventListener(
              "deviceorientation",
              deviceOrientationListener
            );
          } else {
            console.log("no permission for DeviceOrientationEvent");
          }
        });
      } else {
        console.log("no permission required for DeviceOrientationEvent");
        window.addEventListener("deviceorientation", deviceOrientationListener);
      }
      window.addEventListener("deviceorientation", deviceOrientationListener);
    }
  }
  document.getElementById("enable").onclick = enableDeviceOrientation;

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

  var force_x = 0.0;
  var force_y = 0.1;
  var apply_force = false;
  const force_scale = 0.2;
  const force_max = 0.5;
  function clampForce(force) {
    return Math.sign(force) * Math.min(force_max, Math.abs(force));
  }
  const decideForceFn = (event) => {
    event.preventDefault();
    const rect = canvas.getBoundingClientRect();
    const canvas_x = event.clientX - rect.left;
    const canvas_x_proportion = canvas_x / maxX;
    const canvas_y = event.clientY - rect.top;
    const canvas_y_proportion = canvas_y / maxY;
    force_x = clampForce((canvas_x_proportion * 2.0 - 1.0) * force_scale);
    force_y = clampForce(
      -1.0 * (canvas_y_proportion * 2.0 - 1.0) * force_scale
    );
    // console.log("decide force", force_x, force_y);
  };
  canvas.addEventListener("pointerdown", (event) => {
    decideForceFn(event);
    // console.log("start applying force");
    apply_force = true;
  });
  canvas.addEventListener("pointermove", decideForceFn);
  canvas.addEventListener("pointerup", (event) => {
    event.preventDefault();
    // console.log("stop applying force");
    apply_force = false;
  });

  function draw() {
    context.clearRect(0, 0, maxX, maxY);

    context.beginPath();
    context.arc(ball.x, ball.y, ballRadius, 0, 2 * Math.PI);
    context.fill();

    context.beginPath();
    const halfMaxX = maxX / 2.0;
    const halfMaxY = maxY / 2.0;
    context.moveTo(halfMaxX, halfMaxY);
    context.lineTo(
      halfMaxX + (force_x / force_max) * halfMaxX,
      halfMaxY + ((-1.0 * force_y) / force_max) * halfMaxY
    );
    context.lineWidth = 5;
    context.lineCap = "round";
    if (apply_force) {
      context.strokeStyle = "red";
    } else {
      context.strokeStyle = "green";
    }
    context.stroke();
  }

  var start = undefined;
  var lastUpdate = undefined;
  function animate(timestamp) {
    if (start === undefined) {
      start = timestamp;
      lastUpdate = 0;
    } else {
      const elapsed = timestamp - start;
      const elapsedSinceLastUpdate = elapsed - lastUpdate;
      if (apply_force) {
        // console.log("apply force", force_x, force_y);
        sim.set_force(force_x, force_y);
      } else {
        sim.set_force(0.0, 0.0);
      }
      sim.update(elapsedSinceLastUpdate, updateBall);
      lastUpdate = elapsed;
    }
    draw();

    window.requestAnimationFrame(animate);
  }

  window.requestAnimationFrame(animate);
}

await app();
