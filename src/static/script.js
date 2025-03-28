const speedSlider = document.getElementById("speedSlider");
const directionToggle = document.getElementById("directionToggle");
const speedValue = document.getElementById("speedValue");
const directionValue = document.getElementById("directionValue");

const speedMap = {
  0: "Stop",
  1: "Very slow",
  2: "Slow",
  3: "Moderate",
  4: "Fast",
  5: "Very fast",
};

const state = {
  speed: parseInt(speedSlider.value),
  isForward: directionToggle.checked,
};

const render = (state) => {
  speedValue.textContent = speedMap[state.speed];
  directionValue.textContent = state.isForward ? "Forwards" : "Backwards";
};

const debounce = (callback, delay = 150) => {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => callback(...args), delay);
  };
};

const sendControls = (state) => {
  fetch("/control", {
    method: "POST",
    headers: {
      "Content-Type": "application/json; charset=utf-8",
    },
    body: JSON.stringify({
      speed: parseInt(state.speed),
      is_forward: state.isForward,
    }),
  })
    .then((response) => {
      if (!response.ok) {
        throw new Error("Network response was not ok");
      }
      return response.json();
    })
    .then((data) => {
      console.log("Success:", data);
    })
    .catch((error) => {
      console.error("Error:", error);
    });
};

const debouncedSendControls = debounce(sendControls, 150);

speedSlider.addEventListener("input", (event) => {
  state.speed = event.target.value;
  render(state);
  debouncedSendControls(state);
});

directionToggle.addEventListener("change", (event) => {
  state.isForward = event.target.checked;
  render(state);
  debouncedSendControls(state);
});

render(state);
