use structopt::StructOpt;
mod extractbits;
mod convertinput;

#[derive(StructOpt)]
#[structopt(name= "Bitter", about = "Prints active bits of a value")]
struct Opt {
    //#[structopt(short = "c", long="celsius")]
    //celsius: bool,
    #[structopt()]
    value: String,
}

fn main() {
    let opt = Opt::from_args();
    let val = convertinput::conver_to_u32(opt.value);
    let bit_vec = extractbits::extract_active_bits(val);

    for i in bit_vec.iter() {
        print!("{} ", i);
    }
}
