console.log("Hello", "runjs!");

let content = await fetchText(
  "https://www.cgbookcase.com/textures/bark-09.json",
);
console.log("fetched", content);

player.setX(-2);

console.log(getStr("nice"));

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

function tick() {
  // console.log("tick: " + i)
  player.setX(i);
}
