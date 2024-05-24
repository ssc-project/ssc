<div align="center">

[![MIT licensed][license-badge]][license-url]
[![Build Status][ci-badge]][ci-url]
[![Code size][code-size-badge]][code-size-url]

</div>

## Svelte Oxide

Svelte Oxide is a set of tools for Svelte.

The goal is to build a parser, analyzer, transformer, formatter, linter, language server .. all wriiten in Rust.

## Development

Svelte oxide is still in it's early stages of development, a lot of features still need to be implemented before it
is ready for production use.

Here's a feature roadmap:

- [x] AST
- [x] CSS AST
- [ ] CSS Parser
  - [ ] Rule
    - [ ] AtRule
    - [ ] StyleRule
  - [ ] Selector
    - [ ] TypeSelector
    - [ ] IdSelector
    - [ ] ClassSelector
    - [ ] AttributeSelector
    - [ ] PseudoElementSelector
    - [ ] PseudoClassSelector
    - [ ] PercentageSelector
    - [ ] NthSelector
    - [ ] NestingSelector
    - [ ] Combinator
  - [ ] Block
  - [ ] Declaration
- [ ] Parser
  - [ ] Script
  - [ ] StyleSheet
  - [ ] Element
    - [ ] Component
    - [ ] TitleElement
    - [ ] SlotElement
    - [ ] RegularElement
    - [ ] SvelteBody
    - [ ] SvelteComponent
    - [ ] SvelteDocument
    - [ ] SvelteElement
    - [ ] SvelteFragment
    - [ ] SvelteHead
    - [ ] SvelteOptionsRaw
    - [ ] SvelteSelf
    - [ ] SvelteWindow
  - [ ] Block
    - [ ] EachBlock
    - [ ] IfBlock
    - [ ] AwaitBlock
    - [ ] KeyBlock
    - [ ] SnippetBlock
  - [ ] Tag
    - [ ] ExpressionTag
    - [ ] HtmlTag
    - [ ] ConstTag
    - [ ] DebugTag
    - [ ] RenderTag
  - [ ] Text
- [ ] Analyzer
- [ ] Transformer

## License

Svelte oxide is free and open-source software licensed under the [MIT License](./LICENSE).

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/a-rustacean/svelte-oxide/blob/main/LICENSE
[ci-badge]: https://github.com/a-rustacean/svelte-oxide/actions/workflows/ci.yml/badge.svg?event=push&branch=main
[ci-url]: https://github.com/a-rustacean/svelte-oxide/actions/workflows/ci.yml?query=event%3Apush+branch%3Amain
[code-size-badge]: https://img.shields.io/github/languages/code-size/a-rustacean/svelte-oxide
[code-size-url]: https://github.com/a-rustacean/svelte-oxide
