import init, { Simulation } from "../engine/pkg/simple_ball_engine.js";

console.log("Running");

function clampMagnitude(value, max) {
  return Math.sign(value) * Math.min(max, Math.abs(value));
}

function bindPhysicalSensorModel() {
  const sensorModel = {
    sensor_data: {
      initialised: false,
      initial: {
        beta: undefined,
        gamma: undefined,
      },
      current: {
        beta: undefined,
        gamma: undefined,
      },
    },
    force: {
      max: 1.0,
      x: undefined,
      y: undefined,
      apply: false,
    },
  };
  sensorModel.sensor_data.reset = () => {
    sensorModel.sensor_data.initialised = false;
    sensorModel.sensor_data.initial = {
      beta: undefined,
      gamma: undefined,
    };
  };
  function listener(event) {
    const { beta, gamma } = event;
    if (!sensorModel.sensor_data.initialised) {
      sensorModel.sensor_data.initial = {
        beta,
        gamma,
      };
      sensorModel.sensor_data.initialised = true;
    }
    const max_magnitude = 20;
    const beta_diff = clampMagnitude(
      beta - sensorModel.sensor_data.initial.beta,
      max_magnitude
    );
    const gamma_diff = clampMagnitude(
      gamma - sensorModel.sensor_data.initial.gamma,
      max_magnitude
    );
    sensorModel.sensor_data.current = {
      beta,
      gamma,
    };
    sensorModel.force = {
      ...sensorModel.force,
      x: gamma_diff / max_magnitude,
      y: -1.0 * (beta_diff / max_magnitude),
      apply: true,
    };
  }
  window.addEventListener("deviceorientation", listener);
  return sensorModel;
}

async function registerPhysicalForceSensor() {
  console.log("registering physical force sensor");
  if (window.DeviceOrientationEvent) {
    console.log("device supports DeviceOrientationEvent");
    if (DeviceOrientationEvent.requestPermission) {
      console.log("must request permission for DeviceOrientationEvent");
      return DeviceOrientationEvent.requestPermission()
        .then((response) => {
          if (response == "granted") {
            return bindPhysicalSensorModel();
          } else {
            console.log(
              "no permission for DeviceOrientationEvent, response: ",
              response
            );
            return null;
          }
        })
        .catch((error) => {
          console.log(
            "error whilst getting DeviceOrientationEvent permission:",
            error
          );
          return Promise.resolve(null);
        });
    } else {
      console.log("no permission required for DeviceOrientationEvent");
      return Promise.resolve(bindPhysicalSensorModel());
    }
  } else {
    console.log("device does not support DeviceOrientationEvent");
    return Promise.resolve(null);
  }
}

function registerCanvasForceSensor(canvas) {
  const sensorModel = {
    force: {
      max: 0.5,
      x: undefined,
      y: undefined,
      apply: false,
    },
  };
  const force_scale = 0.2;

  const rect = canvas.getBoundingClientRect();

  const decideForceFn = (event) => {
    event.preventDefault();
    const canvas_x = event.clientX - rect.left;
    const canvas_x_proportion = canvas_x / canvas.width;
    const canvas_y = event.clientY - rect.top;
    const canvas_y_proportion = canvas_y / canvas.height;
    sensorModel.force.x = clampMagnitude(
      (canvas_x_proportion * 2.0 - 1.0) * force_scale,
      sensorModel.force.max
    );
    sensorModel.force.y = clampMagnitude(
      -1.0 * (canvas_y_proportion * 2.0 - 1.0) * force_scale,
      sensorModel.force.max
    );
  };
  canvas.addEventListener("pointerdown", (event) => {
    decideForceFn(event);
    sensorModel.force.apply = true;
  });
  canvas.addEventListener("pointermove", decideForceFn);
  canvas.addEventListener("pointerup", (event) => {
    event.preventDefault();
    sensorModel.force.apply = false;
  });
  return sensorModel;
}

function draw(ball, ballRadius, sensorModel, canvas) {
  const { width, height } = canvas;
  const context = canvas.getContext("2d");

  context.clearRect(0, 0, width, height);

  context.beginPath();
  context.arc(ball.x, ball.y, ballRadius, 0, 2 * Math.PI);
  context.fill();

  context.beginPath();
  const max_size_x = (0.8 * width) / 2.0;
  const max_size_y = (0.8 * height) / 2.0;
  const center_x = width / 2.0;
  const center_y = height / 2.0;
  context.moveTo(center_x, center_y);
  const x =
    center_x + (sensorModel.force.x / sensorModel.force.max) * max_size_x;
  const y =
    center_y +
    ((-1.0 * sensorModel.force.y) / sensorModel.force.max) * max_size_y;
  //   console.log(sensorModel, x, y);
  context.lineTo(x, y);
  context.lineWidth = 5;
  context.lineCap = "round";
  if (sensorModel.force.apply) {
    context.strokeStyle = "red";
  } else {
    context.strokeStyle = "green";
  }
  context.stroke();

  document.getElementById(
    "force_apply"
  ).innerText = `${sensorModel.force.apply}`;
  document.getElementById("force_x").innerText = `${sensorModel.force.x}`;
  document.getElementById("force_y").innerText = `${sensorModel.force.y}`;
  document.getElementById("force_max").innerText = `${sensorModel.force.max}`;

  if (sensorModel.sensor_data && sensorModel.sensor_data.initialised) {
    const data = sensorModel.sensor_data;
    document.getElementById("initial_beta").innerText = `${data.initial.beta}`;
    document.getElementById(
      "initial_gamma"
    ).innerText = `${data.initial.gamma}`;
    document.getElementById("current_beta").innerText = `${data.current.beta}`;
    document.getElementById(
      "current_gamma"
    ).innerText = `${data.current.gamma}`;
  }
}

async function app() {
  console.log("starting init ...");
  await init();
  console.log("init done");

  const canvas = document.getElementById("canvas");

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

  var sensorModel = registerCanvasForceSensor(canvas);
  document.getElementById("enable").onclick = async () => {
    const physicalSensorModel = await registerPhysicalForceSensor();
    if (physicalSensorModel !== null) {
      sensorModel = physicalSensorModel;

      const resetButton = document.getElementById("reset");
      resetButton.onclick = sensorModel.sensor_data.reset;
      resetButton.disabled = false;
    }
  };

  var start = undefined;
  var lastUpdate = undefined;
  function animate(timestamp) {
    if (start === undefined) {
      start = timestamp;
      lastUpdate = 0;
    } else {
      const elapsed = timestamp - start;
      const elapsedSinceLastUpdate = elapsed - lastUpdate;
      if (sensorModel.force.apply) {
        // console.log("apply force", sensorModel.force.x, sensorModel.force.y);
        sim.set_force(sensorModel.force.x, sensorModel.force.y);
      } else {
        sim.set_force(0.0, 0.0);
      }
      sim.update(elapsedSinceLastUpdate, updateBall);
      lastUpdate = elapsed;
    }
    draw(ball, ballRadius, sensorModel, canvas);

    window.requestAnimationFrame(animate);
  }

  window.requestAnimationFrame(animate);
}

await app();
