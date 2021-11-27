const { JSDOM } = require('jsdom');
const { NodeHtmlMarkdown } = require('node-html-markdown');
const fs = require("fs/promises");
const {  createWriteStream } = require("fs");
const fetch = require("node-fetch");
const { pipeline } = require('stream/promises');
const replace = require('replace-in-file');

const year = 2020;

async function index(day) {
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

    // download description and create readme
    const dom = await JSDOM.fromURL(`https://adventofcode.com/${year}/day/${day}`);
    const desc = dom.window.document.querySelector(".day-desc").innerHTML;
    const md = NodeHtmlMarkdown.translate(
        desc,
        {
            emDelimiter: "**",
        }
    );

    await fs.writeFile(`${outdir}/README.md`, md);

    // download problem input
    const input = await fetch(`https://adventofcode.com/${year}/day/${day}/input`, {
        headers: {
            'Cookie': `session=${process.env.AOC_SESSION}`
        }
    });
    await pipeline(input.body, createWriteStream(`${outdir}/index.txt`));
}

index(process.argv[2])
