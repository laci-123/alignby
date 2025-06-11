use super::*;


#[test]
fn empty_input() {
    // Empty input produces empty output.
    let aligner = Aligner::new("-");
    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), None);
}

#[test]
fn delimiter_not_in_input() {
    // If the delimiter is nowhere in the input
    // then the output is identical to the input.
    let mut aligner = Aligner::new("-");
    aligner.add_line(String::from("cat dog"), 0);
    aligner.add_line(String::from("elephant     giraffe"), 1);
    aligner.add_line(String::from("123456"), 2);
    aligner.add_line(String::from("árvíztűrő tükörfúrógép"), 3);
    aligner.add_line(String::from(""), 4);

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat dog")));
    assert_eq!(it.next(), Some(String::from("elephant     giraffe")));
    assert_eq!(it.next(), Some(String::from("123456")));
    assert_eq!(it.next(), Some(String::from("árvíztűrő tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("")));
    assert_eq!(it.next(), None);
}

#[test]
fn simple_delimiter() {
    let mut aligner = Aligner::new("-");
    aligner.add_line(String::from("cat-dog"), 0);
    aligner.add_line(String::from("elephant -   giraffe"), 1);
    aligner.add_line(String::from("123 - 456"), 2);
    aligner.add_line(String::from("- árvíztűrő tükörfúrógép"), 3);
    aligner.add_line(String::from("-"), 4);

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat      -dog")));
    assert_eq!(it.next(), Some(String::from("elephant -   giraffe")));
    assert_eq!(it.next(), Some(String::from("123      - 456")));
    assert_eq!(it.next(), Some(String::from("         - árvíztűrő tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("         -")));
    assert_eq!(it.next(), None);
}

#[test]
fn multiple_delimiters() {
    let mut aligner = Aligner::new("-");
    aligner.add_line(String::from("cat-dog-"), 0);
    aligner.add_line(String::from("elephant - giraffe - leopard"), 1);
    aligner.add_line(String::from("123 -- 456"), 2);
    aligner.add_line(String::from("- árvíztűrő-tükörfúrógép"), 3);
    aligner.add_line(String::from("--"), 4);

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
    let mut aligner = Aligner::new("-->");
    aligner.add_line(String::from("cat --> dog"), 0);
    aligner.add_line(String::from("elephant -->   giraffe"), 1);
    aligner.add_line(String::from("123-->456"), 2);
    aligner.add_line(String::from("--> árvíztűrő-tükörfúrógép"), 3);
    aligner.add_line(String::from("-->"), 4);

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat      --> dog")));
    assert_eq!(it.next(), Some(String::from("elephant -->   giraffe")));
    assert_eq!(it.next(), Some(String::from("123      -->456")));
    assert_eq!(it.next(), Some(String::from("         --> árvíztűrő-tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("         -->")));
    assert_eq!(it.next(), None);
}

#[test]
fn multiple_multicharacter_delimiters() {
    let mut aligner = Aligner::new("<>");
    aligner.add_line(String::from("cat<>dog<>"), 0);
    aligner.add_line(String::from("elephant <> giraffe <> leopard"), 1);
    aligner.add_line(String::from("123 <> 456"), 2);
    aligner.add_line(String::from("<> árvíztűrő<>tükörfúrógép"), 3);
    aligner.add_line(String::from("<><>"), 4);

    let mut it = aligner.aligned_lines();
    assert_eq!(it.next(), Some(String::from("cat      <>dog<>")));
    assert_eq!(it.next(), Some(String::from("elephant <> giraffe <> leopard")));
    assert_eq!(it.next(), Some(String::from("123      <> 456")));
    assert_eq!(it.next(), Some(String::from("         <> árvíztűrő<>tükörfúrógép")));
    assert_eq!(it.next(), Some(String::from("         <><>")));
    assert_eq!(it.next(), None);
}
