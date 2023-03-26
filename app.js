await onBegin();

let i = 0;
while (true) {
  try {
    await sleep(16.0);
  } catch (error) {
    console.log(error)
  }

  globalThis.__runtimeInternal.handleEvents()

  tick()
  i++;
}

// ================================================

async function onBegin() {

  let content = await fetchText(
    "https://www.cgbookcase.com/textures/bark-09.json",
  );
  console.log("fetched", content);

  controls.addEventListener("keydown", e => {
    console.log(e.data.key_code)

    if (e.data.key_code === "Space") {
      player.setRandomColor();
    }
  })

}

function tick() {
  player.setPosition(Math.sin(i / 12), 0, Math.cos(i / 12));
}

