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

#[acmd_script( agent = "peach", script = "game_attackhi3", category = ACMD_GAME)]
unsafe fn peach_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 9.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            8.0, 92, 84, 0, 65, 
            5.0, 0.0, 13.0, 2.0, Some(0.0), Some(13.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            10.0, 94, 84, 0, 40, 
            8.0, 0.0, 24.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            }
        /*Active 9-13 → 9-14 (14.0 → 15.0)*/ sv_animcmd::frame(lua_state, 15.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "peach", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn peach_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 1.0);
            if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 1.0, 4.0);
            }
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0, 6.0);
            }
        sv_animcmd::frame(lua_state, 8.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*KBG 105 → 125*/ 7.0, 275, 125, 0, 60, 
            3.8, 0.0, 3.2, 6.5, Some(0.0), Some(2.8), Some(10.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*KBG 105 → 125*/ 7.0, 80, 125, 0, 15, 
            3.8, 0.0, 3.2, 6.5, Some(0.0), Some(2.8), Some(10.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
            }
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0, 3.0);
            }
}

#[acmd_script( agent = "peach", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn peach_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 12.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }for _ in 0..3{;
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*Angle 70° → 367°*/ 2.0, 367, 30, 0, 13, 
            5.5, 0.0, -1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*Angle 70° → 367°*/ 1.0, 367, 30, 0, 13, 
            /*Size 4.5 → 5.5*/ 5.5, 0.0, 4.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::wait(lua_state, 4.0);
            }if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            5.0, 60, 110, 0, 60, 
            6.5, 0.0, -1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            5.0, 60, 110, 0, 60, 
            5.5, 0.0, 4.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 38.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "peach", script = "game_catch", category = ACMD_GAME)]
unsafe fn peach_standinggrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            3.3, 0.0, 8.5, 4.0, Some(0.0), Some(8.5), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            1.65, 0.0, 8.5, 2.35, Some(0.0), Some(8.5), Some(10.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
        /*Active 6-7 → 6-9 (2.0 → 4.0)*/ sv_animcmd::wait(lua_state, 4.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
}

#[acmd_script( agent = "peach", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn peach_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 8.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 9.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            2.6, 0.0, 8.5, 4.0, Some(0.0), Some(8.5), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            1.3, 0.0, 8.5, 2.7, Some(0.0), Some(8.5), Some(11.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
        /*Active 9-10 → 9-11 (2.0 → 3.0)*/ sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
}

#[acmd_script( agent = "peach", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn peach_pivotgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 9.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            3.3, 0.0, 8.5, -4.0, Some(0.0), Some(8.5), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            1.65, 0.0, 8.5, -2.35, Some(0.0), Some(8.5), Some(-16.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
        /*Active 10-11 → 10-12 (2.0 → 3.0)*/ sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        peach_utilt, 
        peach_dtilt, 
        peach_dair, 
        peach_standinggrab, 
        peach_dashgrab, 
        peach_pivotgrab
    );
}
