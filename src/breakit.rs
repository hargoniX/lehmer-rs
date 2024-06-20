use rand::RngCore;

use crate::core::FastU32;

pub fn make_float<R: RngCore>(r: &mut R, max: u32) -> f64 {
    let next = r.next_u32();
    next as f64 / max as f64
}

pub fn breakit<const N: usize>(r: &mut FastU32, max: u32) {
    let mut sample_buf: [f64; N] = [0.0; N];
    for i in 0..N {
        sample_buf[i] = make_float(r, max);
    }

    let mut rtest = FastU32::new(3);
    let mut guess: Option<f64> = None;

    let mut work_buf: [f64; N] = [0.0; N];
    for i in 0..N {
        work_buf[i] = make_float(&mut rtest, max);
    }

    for step in 0..(max - N as u32) {
        if sample_buf == work_buf {
            guess = Some(make_float(&mut rtest, max));
            break;
        } else {
            // TODO: naive
            for i in 0..(N - 1) {
                work_buf[i] = work_buf[i + 1];
            }
            work_buf[N - 1] = make_float(&mut rtest, max);
        }

        if (step % 10_000_000) == 0 {
            println!("Step: {}/{}", step, max);
        }
    }

    match guess {
        None => println!("Failed to find a guess for the seed."),
        Some(x) => println!("My guess for the next float is: {}", x),
    }
}
