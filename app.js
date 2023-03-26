console.log("Hello", "runjs!");

let content = await fetchText(
  "https://www.cgbookcase.com/textures/bark-09.json",
);
// console.log("fetched", content);

player.setX(-2);

console.log(getStr("nice"));

await onBegin();

let i = 0;
while (true) {
  try {
    await sleep(16.0);

  } catch (error) {
    console.log(error)
  }

  globalThis.__runtimeInternal.handleEvents()
  // await fetchText(
  //   "https://www.cgbookcase.com/textures/bark-09.json",
  // );

  tick()
  i++;
}


async function onBegin() {
  controls.addEventListener("keydown", e => {
    console.log(e.data.key_code)

    if (e.data.key_code === "Space") {
      player.setRandomColor();
    }
  })

}

function tick() {
  // console.log("tick: " + i)
  player.setPosition(Math.sin(i / 12), 0, Math.cos(i / 12));

  // let parsed = globalThis.__runtimeInternal.handleEvents()
  // console.log("parsed", parsed)
}

