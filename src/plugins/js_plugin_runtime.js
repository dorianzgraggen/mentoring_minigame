const { core } = Deno;
const { ops } = core;

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

const console = {
  log: (...args) => {
    core.print(`[out]: ${argsToMessage(...args)}\n`, false);
  },
  error: (...args) => {
    core.print(`[err]: ${argsToMessage(...args)}\n`, true);
  },
};

globalThis.console = console;
globalThis.fetchText = async (url) => {
  return core.opAsync("op_fetch", url);
};

globalThis.getStr = (string) => {
  return ops.op_get_str(string);
}

// globalThis.player = {
//   setX: (x) => {
//     core.opSync("op_player_set_x", x);
//   }
// }
