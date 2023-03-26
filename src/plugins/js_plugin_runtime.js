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

function build_command(id, ...args) {
  return ops.op_command(JSON.stringify({ id, args: JSON.stringify(args) }))
}

globalThis.player = {
  setX: (x) => build_command("player_set_x", x),
  setPosition: (x, y, z) => build_command("player_set_position", x, y, z)
}

globalThis.sleep = async (milliseconds) => {
  return core.opAsync("op_sleep", milliseconds);
}
