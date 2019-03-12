#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
extern crate csound;
use csound::*;
use std::fmt::Write;

extern crate rand;
use rand::Rng;
extern crate pancurses;
use pancurses::*;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

const KEY_ESC: i32 = 27;
const KEY_C: i32 = 99;
const KEY_X: i32 = 120;
const KEY_D: i32 = 100;
const KEY_W: i32 = 119;
const KEY_E: i32 = 101;
const KEY_F: i32 = 102;
const KEY_V: i32 = 118;
const KEY_G: i32 = 103;
const KEY_Q: i32 = 113;
const KEY_A: i32 = 97;
const KEY_Z: i32 = 122;
const KEY_B: i32 = 98;
const KEY_O: i32 = 111;

const KEY_SP: i32 = 32;

// MIDI notes
const C: u16 = 60;
const Db: u16 = 61;
const D: u16 = 62;
const Eb: u16 = 63;
const E: u16 = 64;
const F: u16 = 65;
const Gb: u16 = 66;
const G: u16 = 67;
const Ab: u16 = 68;
const A: u16 = 69;
const Bb: u16 = 70;
const B: u16 = 71;
const CC: u16 = 72;

//Nuevo*******************************************************************

enum KeyType{
  MIDI_KEY(u16),
  PERCU(u32),
  NO_SOUND,
  EXIT,
}

#[derive(Default,Debug, Clone)]
struct Note{
  instr_id:u32,
  start: f64,
  duration: f64,
  amplitude: f64,
  midi_keynum: String,
}

impl Note{
  fn new(id: u32, start: f64, duration: f64, amp:f64)->Self{
    let mut note = Self::default();
    note.instr_id = id;
    note.start = start;
    note.duration = duration;
    note.amplitude = amp;
    note
  }
}

fn gen_sound(&mut self, pitch: u16) -> String {
  let mut retval = String::with_capacity(1024);
  self.midi_keynum = Note::midi2pch(pitch);
  writeln!(&mut retval, "i{} {} {} {} {}", self.instr_id, self.start, self.duration, self.amplitude, self.midi_keynum).unwrap();
  retval 
}

fn gen_rhythm(&mut self, instr: u32) - String{
  self.instr_id = instr;
  let mut fng = rand::thread_rng();
  let idamp = rng.gen_range(0.05, 0.75);

let mut retval = String::with_capacity(1024);
  writeln!(&mut retval, "i{} {} {} {}", self.instr_id, self.start, self.duration, idamp).unwrap();
  retval 
}

}


impl KeyType{
  fn parse_key(key: i32) -> (Self, &'static str){
    match key {
      KEY_ESC => (KeyType::EXIT, ""),
            KEY_C => (KeyType::MIDI_KEY(C), "C"),
            KEY_X => (KeyType::MIDI_KEY(Db), "Db"),
            KEY_D => (KeyType::MIDI_KEY(D), "D"),
            KEY_W => (KeyType::MIDI_KEY(Eb), "Eb"),
            KEY_E => (KeyType::MIDI_KEY(E), "E"),
            KEY_F => (KeyType::MIDI_KEY(F), "F"),
            KEY_V => (KeyType::MIDI_KEY(Gb), "Gb"),
            KEY_G => (KeyType::MIDI_KEY(G), "G"),
            KEY_Q => (KeyType::MIDI_KEY(Ab), "Ab"),
            KEY_A => (KeyType::MIDI_KEY(A), "A"),
            KEY_Z => (KeyType::MIDI_KEY(Bb), "Bb"),
            KEY_B => (KeyType::MIDI_KEY(B), "B"),
            KEY_O => (KeyType::MIDI_KEY(CC), "Cc"),
            KEY_SP => (KeyType::PERCU(2), ""),
            _      => (KeyType::NO_SOUND, ""),
    }
  }
}

//Final *********************************************************************

static ORC: &str = "sr=44100
  ksmps=32
  nchnls=2
  0dbfs=1

  ; Instrument #1.
  instr 1
  ipch = cps2pch(p5, 12)
  kenv transegr 0,  .02,  5,  .5,  1, - 3,   0
  aout vco2 p4 * kenv, ipch
  aout moogladder aout, 2000, 0.25
  outs aout, aout
  endin

  ; Instrument #2.
  instr 2
  idamp = p4
  a1  tambourine .8, 0.01, 30, idamp, 0.4
  outs a1 * 0.5, a1 * 0.5
  endin";

//Funcion main

fn main() {

  let (tx, rx):(Sender<String>, Receiver<String>) = mpsc::channel();
  //crear el Thread para el teclado
  let sender = tx.clone();
  
    println!("Hello, world!");
}
