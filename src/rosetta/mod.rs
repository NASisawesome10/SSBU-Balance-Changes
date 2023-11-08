use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash_script::macros;
use smash::app::sv_kinetic_energy;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;

#[acmd_script( agent = "rosetta", script = "game_catch", category = ACMD_GAME)]
unsafe fn rosetta_standinggrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 3.0);
            macros::FT_MOTION_RATE(fighter, 0.5);
        sv_animcmd::frame(lua_state, 5.0);
            macros::FT_MOTION_RATE(fighter, 1.0);
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            3.8, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(10.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            1.9, 0.0, 8.0, 2.1, Some(0.0), Some(8.0), Some(12.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
        /*Active 6-7 → 6-8 (2.0 → 3.0)*/ sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);            
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
        sv_animcmd::frame(lua_state, 20.0);
            macros::FT_MOTION_RATE(fighter, 1.05);
        sv_animcmd::frame(lua_state, 40.0);
            macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "rosetta", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn rosetta_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 3.0);
        macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(lua_state, 5.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 9.0);
        if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
        }
    sv_animcmd::frame(lua_state, 10.0);
        if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 
        3.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        
        macros::CATCH(fighter, 1, Hash40::new("top"), 
        1.5, 0.0, 8.0, 2.5, Some(0.0), Some(8.0), Some(13.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
    /*Active 9-10 → 9-12 (2.0 → 4.0)*/ sv_animcmd::wait(lua_state, 4.0);
        if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
        }
    sv_animcmd::frame(lua_state, 18.0);
        macros::FT_MOTION_RATE(fighter, 1.04);
    sv_animcmd::frame(lua_state, 43.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    smashline::install_acmd_scripts!(
        rosetta_standinggrab, 
        rosetta_dashgrab
    );
}
