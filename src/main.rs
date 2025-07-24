use ams_han_decoder::Result;

fn main() -> Result<()> {
    let args = wild::args_os();
    ams_han_decoder::run(args)
}
