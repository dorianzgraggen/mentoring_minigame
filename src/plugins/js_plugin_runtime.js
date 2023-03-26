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
  setPosition: (x, y, z) => build_command("player_set_position", x, y, z),
  setRandomColor: () => build_command("player_set_color", Math.random() * 360, 1, 0.5)
}

globalThis.sleep = async (milliseconds) => {
  return core.opAsync("op_sleep", milliseconds);
}


globalThis.__runtimeInternal = {
  event_list: { "keydown": [] },
  handleEvents: () => {
    const string = ops.op_get_events_json();
    const events = JSON.parse(string);

    events.forEach(event => {
      let listed_event = globalThis.__runtimeInternal.event_list[event.event_type];

      if (!listed_event) {
        console.error(`Event of type '${event.event_type}' is not yet listed.`);
        return;
      }

      event["type"] = event["event_type"]
      delete event["event_type"];

      event["data"] = JSON.parse(event["data"]);

      listed_event.forEach(listener => listener(event));
    });
  }
}

globalThis.controls = {
  addEventListener: (type, listener) => {
    const event = globalThis.__runtimeInternal.event_list[type];

    if (!event) {
      console.error(`Event of type '${type}' does not exist`);
      return;
    }

    event.push(listener);
  }
}
