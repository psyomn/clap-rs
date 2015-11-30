extern crate clap;

use clap::{App, Arg};

#[test]
fn opts_using_short() {
    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("-f [flag] 'some flag'"),
            Arg::from_usage("-c [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "-f", "some", "-c", "other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_long_space() {
    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag", "some", "--color", "other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_long_equals() {
    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag=some", "--color=other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed() {
    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "-f", "some", "--color", "other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");

    let m = App::new("opts")
        .args(vec![
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag=some", "-c", "other"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_lists() {
    let m = App::new("opts")
        .arg(Arg::with_name("list")
             .short("l")
             .long("list")
             .help("input a list of things")
             .takes_list(true))
        .get_matches();
    // Setting this to false until I have something substantial...
    assert!(false);
}
