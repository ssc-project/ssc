<div align="center">

[![MIT licensed][license-badge]][license-url]
[![Build Status][ci-badge]][ci-url]
[![Code size][code-size-badge]][code-size-url]

</div>

## Svelte Oxide

Svelte Oxide is a set of tools for Svelte.

The goal is to build a parser, analyzer, transformer, formatter, linter, language server .. all wriiten in Rust.

## Development

> [!Warning]
> Svelte Oxide is still in it's early stages of development and should not be used in production code.

A lot of features still need to be implemented before it is ready for production use.

Here's a feature roadmap:

- [x] AST
- [x] CSS AST
- [x] CSS Parser
  - [x] Rule
    - [x] AtRule
    - [x] StyleRule
  - [x] Selector
    - [x] TypeSelector
    - [x] IdSelector
    - [x] ClassSelector
    - [x] AttributeSelector
    - [x] PseudoElementSelector
    - [x] PseudoClassSelector
    - [x] PercentageSelector
    - [x] NthSelector
    - [x] NestingSelector
    - [x] Combinator
  - [x] Block
  - [x] Declaration
- [x] Parser
  - [x] Script
  - [x] Style
  - [x] Element
    - [x] Component
    - [x] TitleElement
    - [x] SlotElement
    - [x] RegularElement
    - [x] SvelteBody
    - [x] SvelteComponent
    - [x] SvelteDocument
    - [x] SvelteElement
    - [x] SvelteFragment
    - [x] SvelteHead
    - [x] SvelteOptionsRaw
    - [x] SvelteSelf
    - [x] SvelteWindow
  - [x] Block
    - [x] EachBlock
    - [x] IfBlock
    - [x] AwaitBlock
    - [x] KeyBlock
    - [x] SnippetBlock
  - [x] Tag
    - [x] ExpressionTag
    - [x] HtmlTag
    - [x] ConstTag
    - [x] DebugTag
    - [x] RenderTag
  - [x] Text
- [ ] Analyzer
- [ ] Transformer

This roadmap just shows which part is implemented, none of the code is properly tested.

## License

Svelte oxide is free and open-source software licensed under the [MIT License](./LICENSE).

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/a-rustacean/svelte-oxide/blob/main/LICENSE
[ci-badge]: https://github.com/a-rustacean/svelte-oxide/actions/workflows/ci.yml/badge.svg?event=push&branch=main
[ci-url]: https://github.com/a-rustacean/svelte-oxide/actions/workflows/ci.yml?query=event%3Apush+branch%3Amain
[code-size-badge]: https://img.shields.io/github/languages/code-size/a-rustacean/svelte-oxide
[code-size-url]: https://github.com/a-rustacean/svelte-oxide
