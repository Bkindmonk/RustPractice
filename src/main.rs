use std::env;
use std::fs::File;
use std::io::prelude::*;

mod sorts;
use crate::sorts::mergesort::mergesort;

fn main() {
    println!("Hello, mergesort!");

//    mergesort(&mut real, &mut scratch);

    let mut vector: Vec<i64> = Vec::new();
    let outputfile = read_vec(&mut vector);
    println!("Vector read: {:?}",vector);

    let mut scratchvec:Vec<i64> = Vec::new();
    scratchvec.clone_from(&vector);

    mergesort(vector.as_mut_slice(),scratchvec.as_mut_slice());

    println!("Vector Sorted: {:?}",vector);
    println!("Output File: {:?}",outputfile);
/*
    let mut counter = 0;
    while counter<real.len(){
        print!("{:?},",real[counter]);
        if counter%10 == 0{
            print!("\n")
        }
        counter+=1;
    }
*/

}

/*
fn output(real:&mut[T], path:String){
    let mut file = File::create(path)?;
    file.write_all()?; //TODO
}
*/

//Reads Command arguments and outputs the array into the space provided
fn read_vec(real: &mut Vec<i64>)->String{
    let args: Vec<String> = env::args().collect();

    let input = &args[1];
    let output = &args[2];

    //Application Name, Input File, Output File
    assert_eq!(args.len(), 3);

    let mut file = File::open(input).unwrap();
    let mut contents:String = String::new();
    file.read_to_string(&mut contents).unwrap();

    let vectored:Vec<&str> = contents.split(',').collect();

    println!("Vector: {:?}, with len {:?}",vectored, vectored.len());
    //let mut vectored_fixed:Vec<i64> = Vec::with_capacity(vectored.len());
    //TODO: Fix New Vector
    let mut vectored_fixed = Vec::with_capacity(vectored.len());
//    let mut vectored_fixed:Vec<i64>=Vec::with_capacity(vectored.len()+1);
    println!("About to enter loop");
    for mut stringnum in vectored {
        println!("Entered Loop");
        stringnum = stringnum.trim();
        println!("trimmed: {:?}",stringnum);
        vectored_fixed.push(stringnum.parse().unwrap());
    }

    *real = vectored_fixed;

    return output.to_string();
}


