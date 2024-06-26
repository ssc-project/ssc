use memchr::memmem::Finder;

use super::{
    cold_branch,
    search::{byte_search, safe_byte_match_table, SafeByteMatchTable},
    source::SourcePosition,
    Kind, Lexer,
};
use crate::diagnostics;

// Irregular line breaks - '\u{2028}' (LS) and '\u{2029}' (PS)
const LS_OR_PS_FIRST: u8 = 0xe2;
const LS_BYTES_2_AND_3: [u8; 2] = [0x80, 0xa8];
const PS_BYTES_2_AND_3: [u8; 2] = [0x80, 0xa9];

static MULTILINE_COMMENT_START_TABLE: SafeByteMatchTable =
    safe_byte_match_table!(|b| matches!(b, b'-' | b'\r' | b'\n' | LS_OR_PS_FIRST));

impl<'a> Lexer<'a> {
    /// Section 12.4 Multi Line Comment
    pub(super) fn skip_comment(&mut self) -> Kind {
        // If `is_on_new_line` is already set, go directly to faster search
        // which only looks for `-->`
        if self.token.is_on_new_line {
            return self.skip_comment_after_line_break(self.source.position());
        }

        byte_search! {
            lexer: self,
            table: MULTILINE_COMMENT_START_TABLE,
            continue_if: (next_byte, pos) {
                // Match found. Decide whether to continue searching.
                if next_byte == b'-' {
                    // SAFETY: Next byte is `-` (ASCII) so after it is UTF-8 char boundary
                    let after_minus = unsafe { pos.add(1) };
                    if after_minus.addr() < self.source.end_addr() {
                        // SAFETY: Have checked there's at least 1 further byte to read
                        if unsafe { after_minus.read() } == b'-' {
                            // SAFETY: Next byte is `-` (ASCII) so after it is UTF-8 char boundary
                            let after_minus2 = unsafe { pos.add(2) };
                            if after_minus2.addr() < self.source.end_addr() {
                                // SAFETY: Have checked there's at least 2 further byte to read
                                if unsafe { after_minus2.read() } == b'>' {
                                    // Consume `-->`
                                    // SAFETY: Consuming `-->` leaves `pos` on a UTF-8 char boundary
                                    pos = unsafe { pos.add(3) };
                                    false
                                } else {
                                    true
                                }
                            } else {
                                // This is last byte in file. Continue to `handle_eof`.
                                // This is illegali, so mark this branch cold.
                                cold_branch(|| true)
                            }
                        } else {
                            true
                        }
                    } else {
                        // This is last byte in file. Continue to `handle_eof`.
                        // This is illegali, so mark this branch cold.
                        cold_branch(|| true)
                    }
                } else if next_byte == LS_OR_PS_FIRST {
                    // `0xE2`. Could be first byte of LS/PS, or could be some other Unicode char.
                    // Either way, Unicode is uncommon, so make this a cold branch.
                    cold_branch(|| {
                        // SAFETY: Next byte is `0xE2` which is always 1st byte of a 3-byte UTF-8 char.
                        // So safe to advance `pos` by 1 and read 2 bytes.
                        let next2 = unsafe {
                            pos = pos.add(1);
                            pos.read2()
                        };
                        if matches!(next2, LS_BYTES_2_AND_3 | PS_BYTES_2_AND_3) {
                            // Irregular line break
                            self.token.is_on_new_line = true;
                            // Ideally we'd go on to `skip_multi_line_comment_after_line_break` here
                            // but can't do that easily because can't use `return` in a closure.
                            // But irregular line breaks are rare anyway.
                        }
                        // Either way, continue searching.
                        // Skip 3 bytes (skipped 1 byte above, macro skips 1 more, so skip 1 more here
                        // to make 3), and continue searching.
                        // SAFETY: `0xE2` is always 1st byte of a 3-byte UTF-8 char,
                        // so consuming 3 bytes will place `pos` on next UTF-8 char boundary.
                        pos = unsafe { pos.add(1) };
                        true
                    })
                } else {
                    // Regular line break.
                    // No need to look for more line breaks, so switch to faster search just for `-->`.
                    self.token.is_on_new_line = true;
                    // SAFETY: Regular line breaks are ASCII, so skipping 1 byte is a UTF-8 char boundary.
                    let after_line_break = unsafe { pos.add(1) };
                    return self.skip_comment_after_line_break(after_line_break);
                }
            },
            handle_eof: {
                self.error(diagnostics::unterminated_multi_line_comment(self.unterminated_range()));
                self.last_comment_end = self.offset();
                return Kind::Eof;
            },
        };

        self.trivia_builder.add_comment(self.token.start, self.offset());
        self.last_comment_end = self.offset();
        Kind::Skip
    }

    fn skip_comment_after_line_break(&mut self, pos: SourcePosition) -> Kind {
        // Can use `memchr` here as only searching for 1 pattern.
        // Cache `Finder` instance on `Lexer` as there's a significant cost to
        // creating it. `Finder::new` isn't a const function, so can't
        // make it a `static`, and `lazy_static!` has a cost each time
        // it's deref-ed. Creating `Finder` unconditionally in `Lexer::new`
        // would be efficient for files containing multi-line comments, but
        // would impose pointless cost on files which don't. So this is
        // the fastest solution.
        if self.comment_end_finder.is_none() {
            self.comment_end_finder = Some(Finder::new("-->"));
        }
        let finder = self.comment_end_finder.as_ref().unwrap();

        let remaining = self.source.str_from_pos_to_end(pos).as_bytes();
        if let Some(index) = finder.find(remaining) {
            // SAFETY: `pos + index + 3` is end of `-->`, so a valid
            // `SourcePosition`
            self.source.set_position(unsafe { pos.add(index + 3) });
            self.trivia_builder.add_comment(self.token.start, self.offset());
            self.last_comment_end = self.offset();
            Kind::Skip
        } else {
            self.source.advance_to_end();
            self.error(diagnostics::unterminated_multi_line_comment(self.unterminated_range()));
            self.last_comment_end = self.offset();
            Kind::Eof
        }
    }
}
