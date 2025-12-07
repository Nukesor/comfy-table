use pretty_assertions::assert_eq;

use comfy_table::*;

#[test]
/// UTF-8 symbols that are longer than a single character are properly handled.
/// This means, that comfy-table detects that they're longer than 1 character and styles/arranges
/// the table accordingly.
fn multi_character_utf8_symbols() {
    let mut table = Table::new();
    table
        .set_header(vec!["Header1", "Header2", "Header3"])
        .add_row(vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row(vec![
            "This is another text",
            "Now\nadd some\nmulti line stuff",
            "✅",
        ]);

    println!("{table}");
    let expected = "
+----------------------+----------------------+------------------------+
| Header1              | Header2              | Header3                |
+======================================================================+
| This is a text       | This is another text | This is the third text |
|----------------------+----------------------+------------------------|
| This is another text | Now                  | ✅                     |
|                      | add some             |                        |
|                      | multi line stuff     |                        |
+----------------------+----------------------+------------------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn multi_character_utf8_word_splitting() {
    let mut table = Table::new();
    table
        .set_width(8)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["test"])
        .add_row(vec!["abc✅def"]);

    println!("{table}");
    let expected = "
+------+
| test |
+======+
| abc  |
| ✅de |
| f    |
+------+";
    println!("{expected}");
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn multi_character_cjk_word_splitting() {
    let mut table = Table::new();
    table
        .set_width(8)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["test"])
        .add_row(vec!["abc新年快乐edf"]);

    println!("{table}");
    let expected = "
+------+
| test |
+======+
| abc  |
| 新年 |
| 快乐 |
| edf  |
+------+";
    println!("{expected}");
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

/// Handle emojis that'd joined via the "zero-width joiner" character U+200D and contain variant
/// selectors.
///
/// Those composite emojis should be handled as a single grapheme and thereby have their width
/// calculated based on the grapheme length instead of the individual chars.
///
/// This is also a regression test, as previously emojis were split in the middle of the joiner
/// sequence, resulting in two different emojis on different lines.
#[test]
fn zwj_utf8_word_splitting() {
    let mut table = Table::new();
    table
        .set_width(8)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["test"])
        .add_row(vec!["ab🙂‍↕️def"]);

    println!("{table}");
    let expected = "
+------+
| test |
+======+
| ab🙂‍↕️ |
| def  |
+------+";
    println!("{expected}");
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

/// Take a few random sentences that apparently caused issues and display them
/// in a table with varying width to test any potential utf-8 glyph splitting issues.
#[test]
fn polish_chars() {
    for width in 0..400 {
        let mut table = Table::new();
        table
            .set_width(width)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .load_preset(presets::ASCII_MARKDOWN)
            .set_header(vec!["ID", "FLAGS", "SUBJECT", "FROM", "DATE"])
            .add_row(vec![
                "54280",
                "",
                "?????????????????????????é???????????????????????????????????????��������????????????????????????????????????????",
                "Google",
                "2025-10-27T18:59:15+00:00",
            ])
            .add_row(vec![
                "54279",
                "",
                "[robotics-worldwide]  [jobs] Postdoctoral Fellow - Wearable Robotics",
                "Priyanshu Agarwal",
                "2025-10-27T22:41:42+05:30",
            ])
            .add_row(vec![
                "54277",
                "",
                "[robotics-worldwide] [meetings] [CFP] IEEE BioRob 2026 | 1-4 August 2026 | Edmonton, Canada | Call for Papers and Workshop/Tutorial Proposals",
                "Mahdi Tavakoli",
                "2025-10-27T10:30:53-06:00",
            ])
            .add_row(vec![
                "54274",
                "",
                "Jak bezpiecznie przechowywać swoje środki? - nowe nagranie z sesji Q&A już dostępne",
                "Portfel Tradera",
                "2025-10-27T17:02:07+01:00",
            ])
            .add_row(vec![
                "54273",
                "",
                "Google Alert - satellite procurement",
                "Google Alerts",
                "2025-10-27T08:42:33-07:00",
            ])
            .add_row(vec![
                "54272",
                "",
                "What is socially acceptable in Denmark that would be horrifying in the U.S.?",
                "Quora Digest",
                "2025-10-27T15:41:06+00:00",
            ])
            .add_row(vec![
                "54271",
                "",
                "Choroby, badania, suplementacja - nowe nagranie z sesji Q&A już dostępne",
                "Portfel Tradera",
                "2025-10-27T13:40:08+01:00",
            ])
            .add_row(vec![
                "54278",
                "",
                "[robotics-worldwide] [News] Application open for HealthTech Master's track with fellowships at the University of Strasbourg",
                "NAGEOTTE Florent",
                "2025-10-27T12:28:58+01:00",
            ])
            .add_row(vec![
                "54276",
                "",
                "[robotics-worldwide] [Jobs] 2 master internships and 1 PhD position in soft robotics and HMI at Inria Lille, France",
                "Quentin Peyron",
                "2025-10-27T10:28:37+01:00",
            ])
            .add_row(vec![
                "54275",
                "",
                "[robotics-worldwide]  [Jobs] Postdoc Opportunity at the University of Bergen, Norway",
                "Morten Fjeld",
                "2025-10-27T08:31:09+00:00",
            ]);

        println!("{table}");
    }
}

/// Test Japanese characters with special symbols and varying widths.
/// This is a regression test for UTF-8 character handling with Japanese text.
#[test]
fn japanese_chars() {
    for width in 0..400 {
        let mut table = Table::new();
        table
            .set_width(width)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .load_preset(presets::ASCII_MARKDOWN)
            .set_header(vec!["ID", "FLAGS", "SUBJECT", "FROM", "DATE"])
            .add_row(vec![
                "106443",
                "*",
                "【九州温泉特集】5,200円～！心と身体に安らぎを",
                "さくらトラベル",
                "2025-01-17 17:19+09:00",
            ]);

        println!("{table}");
    }
}
