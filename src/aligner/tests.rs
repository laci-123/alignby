use super::*;


#[test]
fn empty_input() {
    // Empty input produces empty output.
    let aligner = Aligner::new(Settings { after: false, delimiter: String::from("-") });
    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), None);
}

#[test]
fn delimiter_not_in_input() {
    // If the delimiter is nowhere in the input
    // then the output is identical to the input.
    let mut aligner = Aligner::new(Settings { after: false, delimiter: String::from("-") });
    aligner.add_line(String::from("cat dog"));
    aligner.add_line(String::from("elephant     giraffe"));
    aligner.add_line(String::from("123456"));
    aligner.add_line(String::from("árvíztűrő tükörfúrógép"));
    aligner.add_line(String::from(""));

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat dog")));
    assert_eq!(it.next(), Some(String::from("elephant     giraffe")));
    assert_eq!(it.next(), Some(String::from("123456")));
    assert_eq!(it.next(), Some(String::from("árvíztűrő tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("")));
    assert_eq!(it.next(), None);
}

#[test]
#[should_panic]
fn empty_delimiter() {
    let _aligner = Aligner::new(Settings { after: false, delimiter: String::from("") });
}

#[test]
#[should_panic]
fn empty_delimiter_2() {
    let _aligner = Aligner::new(Settings { after: true, delimiter: String::from("") });
}

#[test]
fn simple_delimiter() {
    let mut aligner = Aligner::new(Settings { after: false, delimiter: String::from("-") });
    aligner.add_line(String::from("cat-dog"));
    aligner.add_line(String::from("elephant -   giraffe"));
    aligner.add_line(String::from("123 - 456"));
    aligner.add_line(String::from("- árvíztűrő tükörfúrógép"));
    aligner.add_line(String::from("-"));
    aligner.add_line(String::from("xyz-"));

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat      -dog")));
    assert_eq!(it.next(), Some(String::from("elephant -   giraffe")));
    assert_eq!(it.next(), Some(String::from("123      - 456")));
    assert_eq!(it.next(), Some(String::from("         - árvíztűrő tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("         -")));
    assert_eq!(it.next(), Some(String::from("xyz      -")));
    assert_eq!(it.next(), None);
}

#[test]
fn multiple_delimiters() {
    let mut aligner = Aligner::new(Settings { after: false, delimiter: String::from("-") });
    aligner.add_line(String::from("cat-dog-"));
    aligner.add_line(String::from("elephant - giraffe - leopard"));
    aligner.add_line(String::from("123 -- 456"));
    aligner.add_line(String::from("- árvíztűrő-tükörfúrógép"));
    aligner.add_line(String::from("--"));

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat      -dog-")));
    assert_eq!(it.next(), Some(String::from("elephant - giraffe - leopard")));
    assert_eq!(it.next(), Some(String::from("123      -- 456")));
    assert_eq!(it.next(), Some(String::from("         - árvíztűrő-tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("         --")));
    assert_eq!(it.next(), None);
}

#[test]
fn mulitcharacter_delimiter() {
    let mut aligner = Aligner::new(Settings { after: false, delimiter: String::from("-->") });
    aligner.add_line(String::from("cat --> dog"));
    aligner.add_line(String::from("elephant -->   giraffe"));
    aligner.add_line(String::from("123-->456"));
    aligner.add_line(String::from("--> árvíztűrő-tükörfúrógép"));
    aligner.add_line(String::from("-->"));
    aligner.add_line(String::from("xyz -->"));

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat      --> dog")));
    assert_eq!(it.next(), Some(String::from("elephant -->   giraffe")));
    assert_eq!(it.next(), Some(String::from("123      -->456")));
    assert_eq!(it.next(), Some(String::from("         --> árvíztűrő-tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("         -->")));
    assert_eq!(it.next(), Some(String::from("xyz      -->")));
    assert_eq!(it.next(), None);
}

#[test]
fn multiple_multicharacter_delimiters() {
    let mut aligner = Aligner::new(Settings { after: false, delimiter: String::from("<>") });
    aligner.add_line(String::from("cat<>dog<>"));
    aligner.add_line(String::from("elephant <> giraffe <> leopard"));
    aligner.add_line(String::from("123 <> 456"));
    aligner.add_line(String::from("<> árvíztűrő<>tükörfúrógép"));
    aligner.add_line(String::from("<><>"));

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat      <>dog<>")));
    assert_eq!(it.next(), Some(String::from("elephant <> giraffe <> leopard")));
    assert_eq!(it.next(), Some(String::from("123      <> 456")));
    assert_eq!(it.next(), Some(String::from("         <> árvíztűrő<>tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("         <><>")));
    assert_eq!(it.next(), None);
}

#[test]
fn align_after_delimiter() {
    let mut aligner = Aligner::new(Settings { after: true, delimiter: String::from(":") });
    aligner.add_line(String::from("cat:dog"));
    aligner.add_line(String::from("elephant: giraffe leopard"));
    aligner.add_line(String::from("123:    456"));
    aligner.add_line(String::from(": árvíztűrő tükörfúrógép"));
    aligner.add_line(String::from("::"));

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat:      dog")));
    assert_eq!(it.next(), Some(String::from("elephant: giraffe leopard")));
    assert_eq!(it.next(), Some(String::from("123:      456")));
    assert_eq!(it.next(), Some(String::from(":         árvíztűrő tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from(":         :")));
    assert_eq!(it.next(), None);
}

#[test]
fn align_after_delimiter_multichar() {
    let mut aligner = Aligner::new(Settings { after: true, delimiter: String::from("->") });
    aligner.add_line(String::from("cat->dog"));
    aligner.add_line(String::from("elephant-> giraffe leopard"));
    aligner.add_line(String::from("123->    456"));
    aligner.add_line(String::from("-> árvíztűrő tükörfúrógép"));
    aligner.add_line(String::from("->>"));

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat->      dog")));
    assert_eq!(it.next(), Some(String::from("elephant-> giraffe leopard")));
    assert_eq!(it.next(), Some(String::from("123->      456")));
    assert_eq!(it.next(), Some(String::from("->         árvíztűrő tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("->         >")));
    assert_eq!(it.next(), None);
}

#[test]
fn multibyte_characters() {
    let mut aligner = Aligner::new(Settings { after: false, delimiter: String::from("#") });
    aligner.add_line(String::from("tűzkő # 123"));
    aligner.add_line(String::from("a # 123"));
    aligner.add_line(String::from("синий кит # 123"));

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("tűzkő     # 123")));
    assert_eq!(it.next(), Some(String::from("a         # 123")));
    assert_eq!(it.next(), Some(String::from("синий кит # 123")));
    assert_eq!(it.next(), None);
}
