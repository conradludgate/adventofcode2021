const { JSDOM } = require('jsdom');
const { NodeHtmlMarkdown } = require('node-html-markdown');
const fs = require("fs/promises");
const { createWriteStream } = require("fs");
const fetch = require("node-fetch");
const { pipeline } = require('stream/promises');
const replace = require('replace-in-file');

const year = 2021;

async function setup(day) {
    console.log("=== SETUP ===");

    let padded = String(day).padStart(2, '0');
    let outdir = `challenges/day${padded}`;

    // create rust project
    await fs.cp("challenges/day00", outdir, {
        recursive: true,
    });

    await replace({
        files: `${outdir}/**/*`,
        from: /(day)00/gi,
        to: (...args) => `${args[1]}${padded}`,
    });

    // download problem input
    const input = await fetch(`https://adventofcode.com/${year}/day/${day}/input`, {
        headers: { 'Cookie': `session=${process.env.AOC_SESSION}` }
    });
    await pipeline(input.body, createWriteStream(`${outdir}/input.txt`));

    await get_description(day);
}

async function get_description(day) {
    console.log("=== UPDATE ===");

    let padded = String(day).padStart(2, '0');
    let outdir = `challenges/day${padded}`;

    // download description and create readme
    const input = await fetch(`https://adventofcode.com/${year}/day/${day}`, {
        headers: { 'Cookie': `session=${process.env.AOC_SESSION}` }
    });
    const dom = new JSDOM(await input.text());

    let md = "";
    dom.window.document.querySelectorAll(".day-desc").forEach(e => md = md + "\n\n" + NodeHtmlMarkdown.translate(e.innerHTML, { emDelimiter: "**" }));

    await fs.writeFile(`${outdir}/README.md`, md);
}

let day = (new Date()).getDate();
if (process.argv.length > 2 && process.argv[2] === "setup") {
    setup(day);
} else if (process.argv.length == 2) {
    get_description(day);
} else {
    console.error("unknown input");
}
