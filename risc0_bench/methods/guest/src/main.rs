#![allow(non_snake_case)] // RFC 1321 uses many capitalized variables
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use risc0_zkvm::guest::env;
use nalgebra::Matrix2;
use std::mem;
//use sodiumoxide::crypto::pwhash::argon2id13;
use std::time::Instant;
use password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2; 

use std::f64::consts::PI;

use std::f64;

const _EPS: f64 = 0.00001;
const _MIN: f64 = f64::MIN_POSITIVE;
const _MAX: f64 = f64::MAX;
const LEFT_ALPHABET_CT: &str = "HXUCZVAMDSLKPEFJRIGTWOBNYQ";
const RIGHT_ALPHABET_PT: &str = "PTLNBQDEOYSFAVZKGJRIHWXUMC";
const ZENITH: usize = 0;
const NADIR: usize = 12;
const SEQUENCE: &str = "WELLDONEISBETTERTHANWELLSAID";




fn main() {

    // read the input
    //let input: u32 = env::read();
    //passhash();
    //draw();
    ray_trace();
    //let array = [1, 5, 3, 4, 7, 13, 24, 56, 12, 54, 10];
    //let result = binary_search(&array, 8);
    //println!("using binary search");
    //let result = fibonacci(0, 1);
    //let result = adder(input);
    //let result = chaocipher();
    //let result = gcd(252, 105);
    //let result = hash("password");
    // write public output to the journal
    //env::commit(&password_hash);
}


/* fn adder(iterations: u32) {
    let mut sum = 0u32;
    for i in 0..iterations {
        sum = sum.wrapping_add(i); 
    }
    std::hint::black_box(sum); // Stops the compiler from optimising the function
}


fn cipher(letter: &char, left: &String, right: &String) -> (usize, char) {
    let pos = right.find(*letter).unwrap();
    let cipher = left.chars().nth(pos).unwrap();
    (pos, cipher)
}

fn chaocipher() -> String{
    let mut left = LEFT_ALPHABET_CT.to_string();
    let mut right = RIGHT_ALPHABET_PT.to_string();

    let ciphertext = SEQUENCE.chars()
        .map(|letter| {
            let (pos, cipher_char) = cipher(&letter, &left, &right);
            left = format!("{}{}", &left[pos..], &left[..pos]);
            left = format!("{}{}{}{}", &left[ZENITH..1], &left[2..NADIR+2], &left[1..2], &left[NADIR+2..]);
            if pos != right.len() - 1 {
                right = format!("{}{}", &right[pos + 1..], &right[..pos + 1]);
            }
            right = format!("{}{}{}{}", &right[ZENITH..2], &right[3..NADIR+2], &right[2..3], &right[NADIR+2..]);
            cipher_char
        })
        .collect::<String>();

    //println!("Plaintext: {}", SEQUENCE);
    //println!("Ciphertext: {}", ciphertext);
    ciphertext
}

fn binary_search<T:PartialOrd>(v: &[T], searchvalue: T) -> Option<T> {
    let mut lower = 0 as usize;
    let mut upper = v.len() - 1;

    while upper >= lower {
        let mid = (upper + lower) / 2;
        if v[mid] == searchvalue {
            return Some(searchvalue);
        } else if searchvalue < v[mid] {
            upper = mid - 1;
        } else {
            lower = mid + 1;
        }
    }

    None
}

fn fibonacci(mut prev: usize, mut curr: usize) {
    mem::swap(&mut prev, &mut curr);
    if let Some(n) = curr.checked_add(prev) {
        println!("{}", n);
        fibonacci(prev, n);
    }
}

fn gcd(m: i32, n: i32) -> i32 {
    if m == 0 {
       n.abs()
    } else {
       gcd(n % m, m)
    }
 } */


/* pub fn hash(passwd: &str) -> (String, argon2id13::HashedPassword) {
    sodiumoxide::init().unwrap();
    let hash = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();
    let texthash = std::str::from_utf8(&hash.0).unwrap().to_string();
    (texthash, hash)
} */

/* pub fn passhash() {
    // Can be: `$argon2`, `$pbkdf2`, or `$scrypt`
    let hash_string = "$argon2i$v=19$m=65536,t=1,p=1$c29tZXNhbHQAAAAAAAAAAA$+r0d29hqEB0yasKr55ZgICsQGSkl0v0kgwhd+U3wyRo";
    let input_password = "password";

    let password_hash = PasswordHash::new(&hash_string).expect("invalid password hash");

    // Trait objects for algorithms to support
    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];

    password_hash.verify_password(algs, input_password).expect("invalid password"); 

} */




// `(f32, f32)` would be faster for some RNGs (including `rand::thread_rng` on 32-bit platforms
// and `rand::weak_rng` as of rand v0.4) as `next_u64` combines two `next_u32`s if not natively
// supported by the RNG.  It would less accurate however.
/* fn is_inside_circle((x, y): (f64, f64)) -> bool {
    x * x + y * y <= 1.0
}

fn simulate<R: Rng>(rng: &mut R, samples: usize) -> f64 {
    let mut count = 0;
    for _ in 0..samples {
        if is_inside_circle(rng.gen()) {
            count += 1;
        }
    }
    (count as f64) / (samples as f64)
}

fn draw() {
    let mut rng = rand::weak_rng();

    println!("Real pi: {}", PI);

    for samples in (3..9).map(|e| 10_usize.pow(e)) {
        let estimate = 4.0 * simulate(&mut rng, samples);
        let deviation = 100.0 * (1.0 - estimate / PI).abs();
        println!("{:9}: {:<11} dev: {:.5}%", samples, estimate, deviation);
    }
} */



#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct Edge {
    pt1: Point,
    pt2: Point,
}

impl Edge {
    fn new(pt1: (f64, f64), pt2: (f64, f64)) -> Edge {
        Edge {
            pt1: Point { x: pt1.0, y: pt1.1 },
            pt2: Point { x: pt2.0, y: pt2.1 },
        }
    }
}

struct Polygon {
    edges: Vec<Edge>, // Polygon has to be created with counter-clockwise coordinates
}

fn pt_in_polygon(pt: &Point, poly: &Polygon) -> bool {
    let count = poly.edges
        .iter()
        .filter(|edge| ray_intersect_seg(pt, edge))
        .count();

    count % 2 == 1
}

fn ray_intersect_seg(p: &Point, edge: &Edge) -> bool {
    let mut pt = p.clone();
    let (mut a, mut b): (&Point, &Point) = (&edge.pt1, &edge.pt2);
    if a.y > b.y {
        std::mem::swap(&mut a, &mut b);
    }
    if pt.y == a.y || pt.y == b.y {
        pt.y += _EPS;
    }

    if (pt.y > b.y || pt.y < a.y) || pt.x > a.x.max(b.x) {
        false
    } else if pt.x < a.x.min(b.x) {
        true
    } else {
        let m_red = if (a.x - b.x).abs() > _MIN {
            (b.y - a.y) / (b.x - a.x)
        } else {
            _MAX
        };
        let m_blue = if (a.x - pt.x).abs() > _MIN {
            (pt.y - a.y) / (pt.x - a.x)
        } else {
            _MAX
        };
        m_blue >= m_red
    }
}

fn ray_trace() {
    let p = |x, y| Point { x, y };
    let testpoints = [p(5.0, 5.0), p(5.0, 8.0), p(-10.0, 5.0), p(0.0, 5.0), p(10.0, 5.0), p(8.0, 5.0), p(10.0, 10.0)];
    let poly_square = Polygon {
        edges: vec![
            Edge::new((0.0, 0.0), (10.0, 0.0)),
            Edge::new((10.0, 0.0), (10.0, 10.0)),
            Edge::new((10.0, 10.0), (0.0, 10.0)),
            Edge::new((0.0, 10.0), (0.0, 0.0)),
        ],
    };
    let poly_square_hole = Polygon {
        edges: vec![
            Edge::new((0.0, 0.0), (10.0, 0.0)),
            Edge::new((10.0, 0.0), (10.0, 10.0)),
            Edge::new((10.0, 10.0), (0.0, 10.0)),
            Edge::new((0.0, 10.0), (0.0, 0.0)),
            Edge::new((2.5, 2.5), (7.5, 2.5)),
            Edge::new((7.5, 2.5), (7.5, 7.5)),
            Edge::new((7.5, 7.5), (2.5, 7.5)),
            Edge::new((2.5, 7.5), (2.5, 2.5)),
        ],
    };
    let poly_strange = Polygon {
        edges: vec![
            Edge::new((0.0, 0.0), (2.5, 2.5)),
            Edge::new((2.5, 2.5), (0.0, 10.0)),
            Edge::new((0.0, 10.0), (2.5, 7.5)),
            Edge::new((2.5, 7.5), (7.5, 7.5)),
            Edge::new((7.5, 7.5), (10.0, 10.0)),
            Edge::new((10.0, 10.0), (10.0, 0.0)),
            Edge::new((10.0, 0.0), (2.5, 2.5)),
        ],
    };
    let poly_hexagon = Polygon {
        edges: vec![
            Edge::new((3.0, 0.0), (7.0, 0.0)),
            Edge::new((7.0, 0.0), (10.0, 5.0)),
            Edge::new((10.0, 5.0), (7.0, 10.0)),
            Edge::new((7.0, 10.0), (3.0, 10.0)),
            Edge::new((3.0, 10.0), (0.0, 5.0)),
            Edge::new((0.0, 5.0), (3.0, 0.0)),
        ],
    };
    print!("\nSquare :");
    for pt in &testpoints {
        print!(" {:?}", pt_in_polygon(pt, &poly_square));
    }
    print!("\nSquare with hole:");
    for pt in &testpoints {
        print!(" {:?}", pt_in_polygon(pt, &poly_square_hole));
    }
    print!("\nStrange polygon :");
    for pt in &testpoints {
        print!(" {:?}", pt_in_polygon(pt, &poly_strange));
    }
    print!("\nHexagon :");
    for pt in &testpoints {
        print!(" {:?}", pt_in_polygon(pt, &poly_hexagon));
    }
    println!();
}

