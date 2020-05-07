/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

pub const MAX: usize = 234_881_024;
pub const P: i64 = 469_762_049;
const G: i64 = 3;
const G_INV: i64 = 156587350;

pub fn fast_pow(a: i64, x: i64) -> i64 {
    let (mut r, mut a_t, mut x_t) = (1, a, x);
    while x_t != 0 {
        if (x_t & 1) == 1 {
            r = (r * a_t) % P;
        }
        a_t = (a_t * a_t) % P;
        x_t >>= 1;
    }
    r
}

pub fn fft(arr: &mut [i64], shift: u8, inv: bool) {
    let n = arr.len();
    let (mut i, mut j) = (1, 0);
    while i < n {
        let mut bit = n >> 1;
        while j >= bit {
            j -= bit;
            bit >>= 1;
        }
        j += bit;
        if i < j {
            arr.swap(i, j);
        }

        i += 1;
    }
    for l in 1..=shift {
        let m = (1 << l) as i64;
        let omega_m = if inv {
            fast_pow(G_INV, (P - 1) / m)
        } else {
            fast_pow(G, (P - 1) / m)
        };
        for k in (0..n).step_by(m as usize) {
            let mut omega = 1;
            for j in 0..((m as usize) >> 1) {
                let t = (omega * arr[k + j + ((m >> 1) as usize)]) % P;
                arr[k + j + ((m >> 1) as usize)] = (arr[k + j] - t) % P;
                arr[k + j] = (arr[k + j] + t) % P;
                omega = (omega * omega_m) % P;
            }
        }
    }
    if inv {
        let n_inv = fast_pow(n as i64, P - 2);
        for i in 0..n {
            arr[i] = (arr[i] * n_inv) % P;
        }
    }
}
