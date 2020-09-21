use clap::{App, Arg};
use std::path::Path;

pub trait Flow {
    fn flow(source: Box<dyn Source>, sink: Box<dyn Sink>) {
        source.from();
        sink.to();
    }
}
pub trait Source {
    fn from(&self) -> ();
}

pub trait Sink {
    fn to(&self) -> ();
}
pub mod fs {
    use crate::{Sink, Source};
    use std::path::Path;

    impl Source for Path {
        fn from(&self) {}
    }
    impl Sink for Path {
        fn to(&self) {}
    }
    // impl Source for FS {
    //     fn sync(from: &Path, to: &Path) -> io::Result<()> {
    //         fs::copy(from, to)?;
    //         Ok(())
    //     }
    // }
}

fn main() {
    let matches = App::new("rusty-data")
        .version("0.0.1")
        .author("rusty-data")
        .about("Dataset splitting supporting multiple data sources")
        .arg(
            Arg::with_name("file")
                .help("Sets the input folder to use")
                .required(true),
        )
        .get_matches();

    let folder = Path::new(matches.value_of("file").unwrap_or(""));

    println!("{:?}", folder);
}
