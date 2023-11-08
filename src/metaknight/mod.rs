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

#[acmd_script( agent = "metaknight", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn metaknight_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 
            /*KBG 100 → 115*/ 10.0, 361, 115, 0, 40, 
            4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 
            /*KBG 100 → 115*/ 10.0, 361, 115, 0, 40, 
            4.5, 0.0, 5.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            /*KBG 100 → 115*/ 10.0, 361, 115, 0, 40, 
            7.5, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 
            /*KBG 100 → 115*/ 7.5, 361, 115, 0, 40, 
            4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 
            /*KBG 100 → 115*/ 7.5, 361, 115, 0, 40, 
            4.0, 0.0, 5.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 
            /*KBG 100 → 115*/ 7.5, 361, 115, 0, 40, 
            4.0, 0.0, 7.7, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            /*KBG 100 → 115*/ 7.5, 361, 115, 0, 40, 
            6.0, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::frame(lua_state, 21.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 40.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "metaknight", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn metaknight_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.5, 367, 100, 90, 0, 
            4.2, 0.0, 4.5, -15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            1.5, 367, 100, 90, 0, 
            4.2, 0.0, 2.5, -9.0, Some(0.0), Some(6.5), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            1.5, 55, 100, 55, 0, 
            4.2, 0.0, 4.5, -15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            1.5, 55, 100, 55, 0, 
            4.2, 0.0, 2.5, -9.0, Some(0.0), Some(6.5), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 13.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.5, 367, 100, 90, 0, 
            4.2, 0.0, 4.5, -15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            1.5, 367, 100, 90, 0, 
            4.2, 0.0, 2.5, -9.0, Some(0.0), Some(6.5), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            1.5, 55, 100, 55, 0, 
            4.2, 0.0, 4.5, -15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            1.5, 55, 100, 55, 0, 
            4.2, 0.0, 2.5, -9.0, Some(0.0), Some(6.5), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 20.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*BKB 30 → 40*/ 4.0, 361, 198, 0, 40, 5.2, 0.0, 7.0, -15.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*BKB 30 → 40*/ 4.0, 361, 198, 0, 40, 5.0, 0.0, 2.5, -9.0, Some(0.0), Some(6.5), Some(-9.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 40.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "metaknight", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn metaknight_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*BKB 30 → 40*/ 4.0, 67, 128, 0, 40, 
            3.5, 0.0, 19.0, 3.5, Some(0.0), Some(19.0), Some(-5.5), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*BKB 30 → 40*/ 4.0, 67, 128, 0, 40, 
            3.0, 0.0, 19.0, 4.5, Some(0.0), Some(16.0), Some(9.5), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            /*BKB 30 → 40*/ 4.0, 50, 128, 0, 40, 
            3.0, 0.0, 19.0, -6.5, Some(0.0), Some(16.0), Some(-11.5), 1.0, 1.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 24.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "metaknight", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn metaknight_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 4.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*BKB 30 → 40*/ 6.0, 35, 108, 0, 40, 
            3.5, 0.0, -6.5, 2.5, Some(0.0), Some(-6.5), Some(-4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*BKB 30 → 40*/ 6.0, 50, 108, 0, 40, 
            3.0, 0.0, -6.5, 3.5, Some(0.0), Some(-4.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            /*BKB 30 → 40*/ 6.0, 50, 108, 0, 40, 
            3.0, 0.0, -6.5, -5.5, Some(0.0), Some(-4.5), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 26.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "metaknight", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn metaknight_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 4.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        sv_animcmd::frame(lua_state, 8.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            4.0, 366, 100, 30, 30, 
            4.5, 0.0, 20.0, -3.8, Some(0.0), Some(20.0), Some(3.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            4.0, 130, 100, 30, 30, 
            4.5, 0.0, 16.0, -10.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            4.0, 130, 100, 30, 30, 
            4.5, 0.0, 16.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 12.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            3.0, 366, 100, 30, 30, 
            4.5, 0.0, 20.0, -3.8, Some(0.0), Some(20.0), Some(3.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            3.0, 130, 100, 30, 30, 
            4.5, 0.0, 16.0, -10.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            3.0, 130, 100, 30, 30, 
            4.5, 0.0, 16.0, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 17.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*BKB 65 → 71*/ 5.0, 85, 148, 0, 71, 
            5.5, 0.0, 17.0, -10.2, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*BKB 65 → 71*/ 5.0, 85, 148, 0, 71, 
            5.5, 0.0, 17.0, 8.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            /*BKB 65 → 71*/ 5.0, 90, 148, 0, 71, 
            6.0, 0.0, 20.0, -3.8, Some(0.0), Some(20.0), Some(3.2), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "metaknight", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn metaknight_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 2.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        sv_animcmd::frame(lua_state, 4.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*KBG 77 → 99*/ 10.0, 35, 99, 0, 50, 
            3.8, 0.0, 4.0, 11.0, Some(0.0), Some(9.8), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 9.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*KBG 78 → 91*/ 13.0, 35, 91, 0, 50, 
            4.6, 0.0, 6.0, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*KBG 78 → 91*/ 13.0, 35, 91, 0, 50, 
            4.6, 0.0, 6.0, -17.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "metaknight", script = "game_specialnspin", category = ACMD_GAME)]
unsafe fn metaknight_nspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*Angle 361° → 75°*/ 12.0, 75, 44, 0, 90, 
            9.0, 0.0, 8.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 
            0.0, 180, 100, 0, 30, 
            15.0, 0.0, 10.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            }
        sv_animcmd::frame(lua_state, 1550.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*Angle 361° → 75°*/ 8.0, 75, 60, 0, 70, 
            9.0, 0.0, 8.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 
            0.0, 180, 100, 0, 30, 
            15.0, 0.0, 10.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            }
}

#[acmd_script( agent = "metaknight", script = "game_specialsdrill", category = ACMD_GAME)]
unsafe fn metaknight_sspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            JostleModule::set_status(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.1, 363, 50, 45, 50, 
            3.0, 0.0, 13.5, 6.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            1.1, 367, 50, 30, 80, 
            3.0, 0.0, 2.0, 6.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            1.1, 120, 50, 45, 50, 
            3.0, 0.0, 13.5, 6.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            1.1, 30, 50, 45, 80, 
            3.0, 0.0, 2.0, 6.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 
            1.1, 367, 50, 0, 25, 
            3.5, 0.0, 10.0, 0.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 
            1.1, 365, 50, 0, 25, 
            4.0, 0.0, 5.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            /*There is now a windbox that is in the middle of all the other current hitboxes with an autolink angle.*/ macros::ATTACK(fighter, 6, 0, Hash40::new("top"), 
            0.0, 367, 50, 0, 25, 
            6.5, 0.0, 8.0, 15.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 21.0);
            if macros::is_excute(fighter) {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            }
}

#[acmd_script( agent = "metaknight", script = "game_specialhi", category = ACMD_GAME)]
unsafe fn metaknight_uspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 8.0);
            if macros::is_excute(fighter) {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
                /*Angle 86° → 82°*/ 9.0, 82, 120, 125, 0, 
                4.0, 0.0, 0.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
                /*Angle 88° → 98°*/ 9.0, 98, 120, 125, 0, 
                4.0, 0.0, 0.0, 14.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
                /*Angle 86° → 82°*/ 9.0, 82, 120, 120, 0, 
                4.0, 0.0, 12.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
                /*Angle 88° → 98°*/ 9.0, 98, 120, 120, 0, 
                4.0, 0.0, 12.0, 14.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 9.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
                6.0, 86, 120, 100, 0, 
                5.0, 0.0, 14.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
                6.0, 86, 120, 100, 0, 
                5.0, 0.0, 2.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                AttackModule::clear(fighter.module_accessor, 2, false);
                AttackModule::clear(fighter.module_accessor, 3, false);

                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
                /*Angle 86° → 100°*/ 6.0, 100, 120, 100, 0, 
                5.0, 0.0, 2.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                AttackModule::clear(fighter.module_accessor, 2, false);
                AttackModule::clear(fighter.module_accessor, 3, false);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 10.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
                6.0, 84, 100, 68, 0, 
                5.0, 0.0, 14.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
                6.0, 84, 100, 68, 0, 
                5.0, 0.0, 2.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                /*There are now two seperate hitboxes, with the old one only hitting aerial opponents and the new grounded one having a different angle.*/
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
                /*Angle 84° → 98°*/ 6.0, 98, 100, 68, 0, 
                5.0, 0.0, 2.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 13.0);
            if macros::is_excute(fighter) {
                AttackModule::clear_all(fighter.module_accessor);
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 22.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 
                6.0, 80, 130, 0, 55, 
                /*Size 7.7 → 8.8*/ 8.8, 0.0, 3.5, -2.0, Some(0.0), Some(10.0), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 
                6.0, 80, 130, 0, 55, 
                /*Size 3.0 → 4.0*/ 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 28.0);
            if macros::is_excute(fighter) {
                AttackModule::clear_all(fighter.module_accessor);
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 48.0);
            if macros::is_excute(fighter) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            }
}

#[acmd_script( agent = "metaknight", script = "game_specialhiloop", category = ACMD_GAME)]
unsafe fn metaknight_uspecialloop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
                6.0, 88, 120, 116, 0, 
                6.0, 0.0, 2.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
                6.0, 86, 120, 110, 0, 
                6.0, 0.0, 14.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
                6.0, 86, 100, 80, 0, 
                6.0, 0.0, 2.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
                6.0, 86, 100, 80, 0, 
                6.0, 0.0, 14.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 5.0);
            if macros::is_excute(fighter) {
                AttackModule::clear_all(fighter.module_accessor);
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 14.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 
                6.0, 80, 118, 0, 55, 
                /*Size 7.7 → 8.8*/ 8.8, 0.0, 3.5, -2.0, Some(0.0), Some(10.0), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                
                macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 
                6.0, 80, 118, 0, 55, 
                /*Size 3.0 → 4.0*/ 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 21.0);
            if macros::is_excute(fighter) {
                AttackModule::clear_all(fighter.module_accessor);
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 41.0);
            if macros::is_excute(fighter) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            }
}

#[acmd_script( agent = "metaknight", script = "game_throwhi", category = ACMD_GAME)]
unsafe fn metaknight_uthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
                /*KBG 76 → 96*/macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 77, 96, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
                macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 7.0);
            if macros::is_excute(fighter) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 39.0);
            if macros::is_excute(fighter) {
                macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 7.0, 65, 150, 0, 85, 9.0, 0.0, 7.0, 2.0, Some(1.0), Some(1.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
                AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 45.0);
            if macros::is_excute(fighter) {
                macros::CHECK_FINISH_CAMERA(fighter, 3, 0);
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER_ADDR, 1.5);
            }
        sv_animcmd::frame(fighter.lua_state_agent, 46.0);
            if macros::is_excute(fighter) {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
                AttackModule::clear_all(fighter.module_accessor);
            }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        metaknight_nair, 
        metaknight_bair, 
        metaknight_uair, 
        metaknight_dair, 
        metaknight_usmash, 
        metaknight_dsmash, 
        metaknight_nspecial, 
        metaknight_sspecial, 
        metaknight_uspecial, 
        metaknight_uspecialloop, 
        metaknight_uthrow
    );
}
