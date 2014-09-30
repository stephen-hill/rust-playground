#![feature(globs)]

extern crate time;
use time::*;

fn main()
{
	println!("Hello {} World!", now());
}