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
    await sleep(1000.0);

  } catch (error) {
    console.log(error)
  }
  // await fetchText(
  //   "https://www.cgbookcase.com/textures/bark-09.json",
  // );

  tick()
  i++;
}


async function onBegin() {
  // controls.addEventListener("keydown", e => {
  //   console.log("key is down lol")
  // })

}

function tick() {
  // console.log("tick: " + i)
  player.setPosition(Math.sin(i / 12), 0, Math.cos(i / 12));

  let parsed = globalThis.__runtimeInternal.handleEvents()
  console.log("parsed", parsed)
}

