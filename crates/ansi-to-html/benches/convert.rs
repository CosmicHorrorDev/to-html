use std::{hint::black_box, io::Read, time::Duration};

use ansi_to_html::Converter;
use divan::{bench, counter::BytesCount, Bencher, Divan};
use flate2::bufread::GzDecoder;

fn main() {
    Divan::default()
        .min_time(Duration::from_millis(500))
        .config_with_args()
        .main();
}

static COMPRESSED_TERMINAL_SESSION: &[u8] = include_bytes!("../assets/terminal_session.gz");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MaybeOpt {
    Optimized,
    Unoptimized,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MaybeEsc {
    Escaped,
    Unescaped,
}

#[bench(args = [
    (MaybeOpt::Optimized, MaybeEsc::Escaped),
    (MaybeOpt::Optimized, MaybeEsc::Unescaped),
    (MaybeOpt::Unoptimized, MaybeEsc::Escaped),
    (MaybeOpt::Unoptimized, MaybeEsc::Unescaped),
])]
fn convert(bencher: Bencher, (opt, esc): (MaybeOpt, MaybeEsc)) {
    let mut decoder = GzDecoder::new(COMPRESSED_TERMINAL_SESSION);
    let mut terminal_session = String::new();
    decoder.read_to_string(&mut terminal_session).unwrap();

    let bytes_counter = BytesCount::of_str(&terminal_session);

    let converter = Converter::new()
        .skip_escape(esc == MaybeEsc::Unescaped)
        .skip_optimize(opt == MaybeOpt::Unoptimized);

    bencher
        .counter(bytes_counter)
        .bench(|| converter.convert(black_box(&terminal_session)).unwrap());
}
