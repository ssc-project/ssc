<div align="center">

[![MIT licensed][license-badge]][license-url]
[![Build Status][ci-badge]][ci-url]
[![Code size][code-size-badge]][code-size-url]
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fssc-project%2Fssc.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fssc-project%2Fssc?ref=badge_shield)

</div>

## SSC

SSC (stands for `Speedy Svelte Compiler`) is a super-fast Svelte compiler written in Rust.

The goal is to build a parser, analyzer, transformer, formatter, linter, language server .. all wriiten in Rust.

## Development

> [!Warning]
> SSC is still in it's early stages of development and should not be used in production code.

A lot of features still need to be implemented before it is ready for production use.

Here's a feature roadmap:

- [x] CSS AST
- [x] CSS Parser
- [x] CSS Analyzer
- [x] CSS Transformer
- [x] CSS Printer (codegen)
- [x] AST
- [x] Parser
- [ ] Analyzer
- [ ] Transformer
- [x] Printer (codegen)

This roadmap just shows which part is implemented, none of the code is properly tested.

## License

SSC is free and open-source software licensed under the [MIT License](./LICENSE).

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/ssc-project/ssc/blob/main/LICENSE
[ci-badge]: https://github.com/ssc-project/ssc/actions/workflows/ci.yml/badge.svg?event=push&branch=main
[ci-url]: https://github.com/ssc-project/ssc/actions/workflows/ci.yml?query=event%3Apush+branch%3Amain
[code-size-badge]: https://img.shields.io/github/languages/code-size/ssc-project/ssc
[code-size-url]: https://github.com/ssc-project/ssc


[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fssc-project%2Fssc.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fssc-project%2Fssc?ref=badge_large)