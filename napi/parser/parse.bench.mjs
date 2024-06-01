import {fileURLToPath} from 'url';
import {join as pathJoin} from 'path';
import {readFile, writeFile} from 'fs/promises';
import {Bench} from 'tinybench';
import {parseSync} from './index.js';

const IS_CI = !!process.env.CI,
    ACCURATE = IS_CI || process.env.ACCURATE;

const urls = [
    'https://raw.githubusercontent.com/sveltejs/learn.svelte.dev/766e768fd0de3168c37c297e41162349f0a8f8a6/src/routes/tutorial/%5Bslug%5D/Menu.svelte',
    'https://raw.githubusercontent.com/sveltejs/learn.svelte.dev/766e768fd0de3168c37c297e41162349f0a8f8a6/src/routes/tutorial/%5Bslug%5D/Editor.svelte',
    'https://raw.githubusercontent.com/sveltejs/learn.svelte.dev/766e768fd0de3168c37c297e41162349f0a8f8a6/src/routes/tutorial/%5Bslug%5D/Output.svelte',

];

// Same directory as Rust benchmarks use for downloaded files
const cacheDirPath = pathJoin(fileURLToPath(import.meta.url), '../../../target');

const files = await Promise.all(urls.map(async (url) => {
    const filename = url.split('/').at(-1),
        path = pathJoin(cacheDirPath, filename);

    let code;
    try {
        code = await readFile(path, 'utf8');
        if (IS_CI) console.log('Found cached file:', filename);
    } catch {
        if (IS_CI) console.log('Downloading:', filename);
        const res = await fetch(url);
        code = await res.text();
        await writeFile(path, code);
    }

    return {filename, code};
}));

const bench = new Bench(
    ACCURATE
    ? {
        warmupIterations: 20, // Default is 5
        time: 5000, // 5 seconds, default is 500 ms
        iterations: 100, // Default is 10
    }
    : undefined
);

for (const {filename, code} of files) {
    bench.add(`parser_napi[${filename}]`, () => {
        const res = parseSync(code);
        JSON.parse(res.root);
    });
}

console.log('Warming up');
await bench.warmup();
console.log('Running benchmarks');
await bench.run();
console.table(bench.table());

// If running on CI, save results to file
if (IS_CI) {
    const dataDir = process.env.DATA_DIR;
    const results = bench.tasks.map(task => ({
        filename: task.name.match(/\[(.+)\]$/)[1],
        duration: task.result.period / 1000, // In seconds
    }));
    await writeFile(pathJoin(dataDir, 'results.json'), JSON.stringify(results));
}
