/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

/*
//
// Tackler-Mk1 ANTLR Lexer and Grammars
//
ID: NameStartChar (NameChar)*;

SUBID: (NameStartChar | DIGIT) (NameChar)*;

fragment
NameChar
   : NameStartChar
   | DIGIT
   | '_'
   | '-'
   | '\u00B7'
   | '\u0300'..'\u036F'
   | '\u203F'..'\u2040'
   ;

fragment
NameStartChar
   : '$' | '¢' | '£' | '¤' | '¥' // common currency symbols which are not in block 20A0-20CF
   | '\u00B5' //  Micro Sign
   | '\u00B9' | '\u00B2' | '\u00B3' // Superscript 1, 2, 3 (Latin-1 Supplement)
   | '\u00B0' // Degree Sign
   | '\u00BC' | '\u00BD' | '\u00BE' // Vulgar Fraction: 1/4, 1/2, 3/4 (Latin-1 Supplement)
   | 'A'..'Z' | 'a'..'z'
   | '\u00C0'..'\u00D6'
   | '\u00D8'..'\u00F6'
   | '\u00F8'..'\u02FF'
   | '\u0370'..'\u037D'
   | '\u037F'..'\u1FFF'
   | '\u200C'..'\u200D'
   | '\u2070'..'\u218F'
   | '\u2C00'..'\u2FEF'
   | '\u3001'..'\uD7FF'
   | '\uF900'..'\uFDCF'
   | '\uFDF0'..'\uFFFD'
   ;
 */

#[inline]
pub(crate) fn id_char(c: char) -> bool {
    id_start_char(c)
        | matches!(
            c,
            |'0'..='9' // Ascii Numbers
            | '_' | '-' | '\u{00B7}' // Middle Dot
            | '\u{0300}'..='\u{036F}' // Combining Diacritical Marks
            | '\u{203F}'..='\u{2040}' // Undertie, Character Tie
        )
}

#[inline]
pub(crate) fn id_start_char(c: char) -> bool {
    matches!(c,
        'a'..='z'
        | 'A'..='Z'
        | '$' | '¢' | '£' | '¤' | '¥' // common currency symbols which are not in block 20A0-20CF
        | '\u{00B0}' // Degree Sign
        | '\u{00B5}' // Micro Sign
        | '\u{00B9}' | '\u{00B2}' | '\u{00B3}' // Superscript 1, 2, 3 (Latin-1 Supplement)
        | '\u{00BC}' | '\u{00BD}' | '\u{00BE}' // Vulgar Fraction: 1/4, 1/2, 3/4 (Latin-1 Supplement)
        | '\u{00C0}'..='\u{00D6}'
        | '\u{00D8}'..='\u{00F6}'
        | '\u{00F8}'..='\u{02FF}'
        | '\u{0370}'..='\u{037D}'
        | '\u{037F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
    )
}

/// Whitespace (simple space and tab)
#[inline]
pub(crate) fn space_char(c: char) -> bool {
    matches!(c, |' '| '\t')
}

/// Assorted collection of Punctuation characters
#[rustfmt::skip]
#[inline]
pub(crate) fn punct_char(c: char) -> bool {
    c.is_ascii_punctuation()
        || matches!(c,
            | '\u{00A0}'..='\u{00BF}' // Latin-1 Punctuation and Symbols
            | '\u{00D7}' // Multiplication sign
            | '\u{00F7}' // Division sign
            | '\u{037E}' // Greek Question Mark
            )
}

#[inline]
pub(crate) fn other_char(c: char) -> bool {
    matches!(
        c,
        | '\u{2000}'..='\u{206F}' // General Punctuation

        | '\u{2190}'..='\u{2BFF}'
        /*
        2190..21FF; Arrows
        2200..22FF; Mathematical Operators
        2300..23FF; Miscellaneous Technical
        2400..243F; Control Pictures
        2440..245F; Optical Character Recognition
        2460..24FF; Enclosed Alphanumerics
        2500..257F; Box Drawing
        2580..259F; Block Elements
        25A0..25FF; Geometric Shapes
        2600..26FF; Miscellaneous Symbols
        2700..27BF; Dingbats
        27C0..27EF; Miscellaneous Mathematical Symbols-A
        27F0..27FF; Supplemental Arrows-A
        2800..28FF; Braille Patterns
        2900..297F; Supplemental Arrows-B
        2980..29FF; Miscellaneous Mathematical Symbols-B
        2A00..2AFF; Supplemental Mathematical Operators
        2B00..2BFF; Miscellaneous Symbols and Arrows
         */

        | '\u{3000}' // Ideographic Space

        | '\u{1D400}'..='\u{1D7FF}' // Mathematical Alphanumeric Symbols

        | '\u{1EE00}'..='\u{1FAFF}'
        /*
        1EE00..1EEFF; Arabic Mathematical Alphabetic Symbols
        1F000..1F02F; Mahjong Tiles
        1F030..1F09F; Domino Tiles
        1F0A0..1F0FF; Playing Cards
        1F100..1F1FF; Enclosed Alphanumeric Supplement
        1F200..1F2FF; Enclosed Ideographic Supplement
        1F300..1F5FF; Miscellaneous Symbols and Pictographs
        1F600..1F64F; Emoticons
        1F650..1F67F; Ornamental Dingbats
        1F680..1F6FF; Transport and Map Symbols
        1F700..1F77F; Alchemical Symbols
        1F780..1F7FF; Geometric Shapes Extended
        1F800..1F8FF; Supplemental Arrows-C
        1F900..1F9FF; Supplemental Symbols and Pictographs
        1FA00..1FA6F; Chess Symbols
        1FA70..1FAFF; Symbols and Pictographs Extended-A
         */
    )
}

#[inline]
pub(crate) fn content_char(c: char) -> bool {
    c.is_alphanumeric()
        || space_char(c)
        || punct_char(c)
        || id_char(c)
        || other_char(c)
        || id_start_char(c) // This is mostly overlapping with alphanumeric and others above
}
