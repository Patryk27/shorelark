import * as sim from "lib-simulation-wasm";
import { TerminalStdinComponent } from "./app/components/terminal-stdin";
import { TerminalStdoutComponent } from "./app/components/terminal-stdout";
import { ViewportComponent } from "./app/components/viewport";

const stdout = new TerminalStdoutComponent(document.getElementById("terminal-stdout"));
const stdin = new TerminalStdinComponent(document.getElementById("terminal-stdin"));
const viewport = new ViewportComponent(document.getElementById("viewport"));

/**
 * Current simulation's parameters.
 *
 * @type {Config}
 */
let config = new sim.Config();

/**
 * Current simulation.
 *
 * @type {Simulation}
 */
let simulation = new sim.Simulation(config);

/**
 * Whether the simulation is working or not.
 * Can be modified by the `pause` command.
 *
 * @type {boolean}
 */
let active = true;

stdout.println("  _____ _                    _            _    ");
stdout.println(" / ____| |                  | |          | |   ");
stdout.println("| (___ | |__   ___  _ __ ___| | __ _ _ __| | __");
stdout.println(" \\___ \\| '_ \\ / _ \\| '__/ _ \\ |/ _` | '__| |/ /");
stdout.println(" ____) | | | | (_) | | |  __/ | (_| | |  |   < ");
stdout.println("|_____/|_| |_|\\___/|_|  \\___|_|\\__,_|_|  |_|\\_\\");
stdout.println("");
stdout.println("Simulation of life & evolution - powered by neural networks, genetic algorithms, and high-school math.");
stdout.println("");
stdout.println("Blog post with details coming!");
stdout.println("");
stdout.println("# About");
stdout.println("");
stdout.println("Each white triangle represents a single bird with its eyesight; each bird has a brain that decides where and how fast the bird should move.");
stdout.println("");
stdout.println("Each hacker-green circle represents a single unit of food (pizza, so to say), which birds are meant to find and eat.");
stdout.println("");
stdout.println("After 2500 turns (around a minute without fast-forwarding), birds with the best fitness scores (i.e. the ones who ate the most) are reproduced and their offspring starts the simulation anew.");
stdout.println("");
stdout.println("Each generation, thanks to evolution, gets slightly better at locating the food - it's almost as if the birds programmed themselves!");
stdout.println("");
stdout.println("Note that this represents neither a swarm intelligence (as birds don't see each other), nor a boids simulation (as birds are not hard-coded to find the food) - just regular neural network & genetic algorithm magic.");
stdout.println("");
stdout.println("Apart from being a bare observer, you can also affect the simulation by entering commands in the input below (`reset` is of particular interest as it allows to alter simulation parameters).");
stdout.println("");
stdout.println("Source code (Rust & JS) is available at: https://github.com/Patryk27/shorelark");
stdout.println("");
stdout.println("Have fun!");
stdout.println("");
stdout.println("# Command cheatsheet");
stdout.println("");
stdout.println("## (p)ause");
stdout.println("Pauses (or resumes) the simulation");
stdout.println("");
stdout.println(`## (r)eset [animals=${config.animals}] [f=${config.foods}] [...]`);
stdout.println("Restarts the entire simulation from scratch with given optional parameters:");
stdout.println("");
stdout.println("- (a)nimals: int");
stdout.println("  number of simulated animals on the board");
stdout.println(`  (default: ${config.animals})`);
stdout.println("");
stdout.println("- (f)oods: int");
stdout.println("  number of simulated foods on the board");
stdout.println(`  (default: ${config.foods})`);
stdout.println("");
stdout.println("- (n)eurons: int");
stdout.println("  number of brain neurons per each animal");
stdout.println(`  (default: ${config.neurons})`);
stdout.println("");
stdout.println("- (p)hotoreceptors: int");
stdout.println("  number of eye cells per each animal");
stdout.println(`  (default: ${config.photoreceptors})`);
stdout.println("");
stdout.println("## (t)rain");
stdout.println("Fast-forwards an entire generation, allowing to see the outcome faster.");
stdout.println("");
stdout.scrollToTop();

stdin.onInput((cmd) => {
    stdout.println("");
    stdout.println("$ " + cmd);

    try {
        exec(cmd);
    } catch (err) {
        stdout.println(`  ^ err: ${err}`);
    }
});

function exec(cmd) {
    if (cmd === "p" || cmd === "pause") {
        execPause();
        return;
    }

    if (cmd === "r" || cmd === "reset" || cmd.startsWith("r ") || cmd.startsWith("reset ")) {
        execReset(cmd);
        return;
    }

    if (cmd === "t" || cmd === "train") {
        execTrain();
        return;
    }

    throw "unknown command";
}

function execPause() {
    active = !active;
}

function execReset(cmd) {
    let newConfig = new sim.Config();

    const argsList = cmd.split(" ");
    argsList.shift();

    for (const arg of argsList) {
        const [argName, argValue] = arg.split("=");

        switch (argName) {
            case "a":
            case "animals":
                newConfig.animals = parseInt(argValue);
                break;

            case "f":
            case "foods":
                newConfig.foods = parseInt(argValue);
                break;

            case "n":
            case "neurons":
                newConfig.neurons = parseInt(argValue);
                break;

            case "p":
            case "photoreceptors":
                newConfig.photoreceptors = parseInt(argValue);
                break;

            default:
                throw `unknown parameter: ${argName}`;
        }
    }

    config = newConfig;
    simulation = new sim.Simulation(config);
    active = true;
}

function execTrain() {
    const stats = simulation.train();
    stdout.println(stats);
    active = true;
}

function loop() {
    if (active) {
        const stats = simulation.step();
        const world = simulation.world();

        if (stats.length > 0) {
            stdout.println(stats);
        }

        viewport.render(world);
    }

    requestAnimationFrame(loop);
}

requestAnimationFrame(loop);
