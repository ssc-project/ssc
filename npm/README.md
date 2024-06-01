# SSC

See index.d.ts for `parseSync` and `parseAsync` API.

## ESM

```javascript
import ssc from "ssc-parser";
import assert from "assert";

function test(ret) {
  const root = JSON.parse(ret.root);
  assert(roo.fragment.nodes.length == 1);
  assert(ret.errors.length == 0);
}

const sourceText = "<p>Hello, World</p>";

test(ssc.parseSync(sourceText));

async function main() {
  test(await ssc.parseAsync(sourceText));
}

main();
```
