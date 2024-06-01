import ssc from './index.js';
import assert from 'assert';

console.log(`Testing on ${process.platform}-${process.arch}`)

function test(ret) {
  console.log(ret);
  assert(JSON.parse(ret.root).fragment.nodes.length == 1);
  assert(ret.errors.length == 0);
  assert(ret.comments.length == 1);
}

const sourceText = "<!-- comment --> foo";

test(ssc.parseSync(sourceText));

async function main() {
  test(await ssc.parseAsync(sourceText));
}

main()
