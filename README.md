# Svelte Oxide

The Svelte compiler, rewritten in Rust.

## Description

This projects aims to make `Svelte` usable without `Node.js` and make the compiler _blazingly fast_.

# Development

A lot of features still need to be implemented before `svelte-oxide` is ready for production use.

Feature roadmap:

- [ ] Parser
  - [x] Comment
  - [x] Script
  - [x] StyleSheet
  - [x] Element
    - [x] Component
    - [x] Title
    - [x] Slot
    - [x] Regular
    - [x] SvelteBody
    - [x] SvelteComponent
    - [x] SvelteDocument
    - [x] SvelteElement
    - [x] SvelteFragment
    - [x] SvelteHead
    - [x] SvelteOptionsRaw
    - [x] SvelteSelf
    - [x] SvelteWindow
  - [ ] Block
    - [ ] Each
    - [x] If
    - [ ] Await
    - [ ] Key
    - [ ] Snippet
  - [x] Tag
    - [x] Expression
    - [x] Html
    - [x] Const
    - [x] Debug
    - [x] Render
  - [x] Text
- [ ] Analyzer
- [ ] Transformer

## License

This project is licensed under the MIT License - see the [LICENSE] file for details
