#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

// 64
    mod mario;
    mod donkey;
    mod link;
    mod samus;
    mod samusd;
    mod yoshi;
    mod kirby;
    mod fox;
    mod pikachu;
    mod luigi;
    mod ness;
    mod captain;
    mod purin;

// Melee
    mod peach;
    mod daisy;
    mod koopa;
    mod popo;
    mod sheik;
    mod zelda;
    mod mariod;
    mod pichu;
    mod falco;
    mod marth;
    mod lucina;
    mod younglink;
    mod ganon;
    mod mewtwo;
    mod roy;
    mod chrom;
    mod gamewatch;

// Brawl
    //mod metaknight;
    //mod pit;
    //mod pitb;
    //mod szerosuit;
    //mod wario;
    //mod snake;
    //mod ike;
    //mod pzenigame;
    //mod pfushigisou;
    //mod plizardon;
    //mod lucas;
    //mod sonic;
    mod dedede;
    //mod pikmin;
    //mod lucario;
    //mod robot;
    //mod toonlink;
    //mod wolf;

//Smash 4
    //mod murabito;
    //mod wiifit;
    mod rosetta;
    mod littlemac;
    //mod gekkouga;
    //mod palutena;
    //mod pacman;
    //mod reflect;
    //mod shulk;
    mod koopajr;
    //mod duckhunt;
    //mod ryu;
    //mod ken;
    //mod cloud;
    mod kamui;
    //mod bayonetta;

// Ultimate
    //mod inkling;
    mod ridley;
    //mod simon;
    //mod richter;
    mod krool;
    //mod shizue;
    //mod gaogaen;
    //mod packun;
    //mod jack;
    //mod brave;
    //mod buddy;
    //mod dolly;
    //mod master;
    //mod tantan;
    //mod pickel;
    //mod edge;
    //mod eflame;
    //mod elight;
    //mod kazuya;
    //mod trail;

mod custom;

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
    pub static FIGHTER_CUTIN_MANAGER_ADDR: *mut smash::app::FighterCutInManager;
  }

#[skyline::main(name = "balance_changes")]
pub fn main() {
// 64
    mario::install();
    donkey::install();
    link::install();
    samus::install();
    samusd::install();
    yoshi::install();
    kirby::install();
    fox::install();
    pikachu::install();
    luigi::install();
    ness::install();
    captain::install();
    purin::install();

// Melee
    peach::install();
    daisy::install();
    koopa::install();
    popo::install();
    sheik::install();
    zelda::install();
    mariod::install();
    pichu::install();
    //falco::install();
    marth::install();
    lucina::install();
    younglink::install();
    ganon::install();
    mewtwo::install();
    roy::install();
    chrom::install();
    gamewatch::install();

// Brawl
    //metaknight::install();
    //pit::install();
    //pitb::install();
    //szerosuit::install();
    //wario::install();
    //snake::install();
    //ike::install();
    //pzenigame::install();
    //pfushigisou::install();
    //plizardon::install();
    //lucas::install();
    //sonic::install();
    dedede::install();
    //pikmin::install();
    //lucario::install();
    //robot::install();
    //toonlink::install();
    //wolf::install();

// Sm4sh
    //murabito::install();
    //wiifit::install();
    rosetta::install();
    littlemac::install();
        littlemac::install_agent_code();
    //gekkouga::install();
    //palutena::install();
    //pacman::install();
    //reflect::install();
    //shulk::install();
    koopajr::install();
        koopajr::install_agent_code();
    //duckhunt::install();
    //ryu::install();
    //ken::install();
    //cloud::install();
    kamui::install();
    //bayonetta::install();

// Ultimate
    //inkling::install();
    ridley::install();
    //simon::install();
    //richter::install();
    krool::install();
    //shizue::install();
    //gaogaen::install();
    //packun::install();
    //jack::install();
    //brave::install();
    //buddy::install();
    //dolly::install();
    //master::install();
    //tantan::install();
    //pickel::install();
    //edge::install();
    //eflame::install();
    //elight::install();
    //kazuya::install();
    //trail::install();

    // Config
    custom::install();
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}