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

#[acmd_script( agent = "pikachu", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn pikachu_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        sv_animcmd::frame(lua_state, 11.0);
            if macros::is_excute(fighter) {
            /*There is now a big hitbox between the four current ones with the highest priority.*/ macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.4, 367, 100, 50, 0, 
            5.4, 0.0, 5.5, 9.5, Some(0.0), Some(5.5), Some(10.0), 0.5, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            1.4, 180, 100, 75, 0, 
            1.7, 0.0, 7.5, 15.5, Some(0.0), Some(2.8), Some(15.5), 0.5, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            
            /*
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*FKB 50 → 80*/ 1.4, 35, 100, 80, 0, 
            /*Z 9.5 → 5.5*/ 1.7, 0.0, 1.5, 5.5, Some(0.0), Some(1.5), Some(8.0), 0.5, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            /*FKB 45 → 75*/1.4, 290, 100, 75, 0, 
            /*Z 9.5 → 5.5*/ 1.7, 0.0, 9.5, 5.5, Some(0.0), Some(9.5), Some(8.0), 0.5, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            1.4, 60, 100, 35, 0, 
            1.7, 0.0, 1.5, 13.0, Some(0.0), Some(1.5), Some(9.5), 0.5, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 
            1.4, 280, 100, 35, 0, 
            1.7, 0.0, 9.5, 13.0, Some(0.0), Some(9.5), Some(9.5), 0.5, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            }
            */
            }
        sv_animcmd::frame(lua_state, 26.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 27.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            4.8, 48, 154, 0, 40, 
            7.0, 0.0, 5.5, 11.0, Some(0.0), Some(5.5), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 32.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "pikachu", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn pikachu_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 4.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }for _ in 0..5{;
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 
            1.0, 367, 100, 40, 0, 
            4.2, 0.0, 3.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 
            1.0, 367, 100, 40, 0, 
            4.2, 0.0, 3.5, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::wait(lua_state, 2.0);
            }if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 
            3.6, 45, 160, 0, 40, 
            5.0, 0.0, 7.0, 6.0, Some(0.0), Some(7.0), Some(-4.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        /*Autocancels 1-3 38 onward → 1-3 and 41 onward (38.0 → 41.0)*/ sv_animcmd::frame(lua_state, 41.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "pikachu", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn pikachu_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        sv_animcmd::frame(lua_state, 15.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            15.0, 45, 73, 0, 50, 
            2.5, 0.0, 7.0, 9.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            }
        sv_animcmd::frame(lua_state, 16.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            15.0, 45, 73, 0, 50, 
            2.5, 0.0, 7.0, 11.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            }
        sv_animcmd::frame(lua_state, 17.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            18.0, 45, 75, 0, 50, 
            3.6, 0.0, 7.0, 14.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            }
        sv_animcmd::frame(lua_state, 18.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            18.0, 45, 75, 0, 60, 
            3.6, 0.0, 7.0, 18.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            }
        sv_animcmd::frame(lua_state, 20.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*KBG 102 → 96*/ 12.0, 45, 96, 0, 40, 
            5.2, 0.0, 7.0, 23.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            }
        sv_animcmd::frame(lua_state, 24.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*KBG 102 → 96*/ 12.0, 45, 96, 0, 40, 
            4.0, 0.0, 7.0, 25.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            }
        /*Active 15-16/17-19/20-29 → 15-16/17-19/20-24 (30.0 → 25.0)*/ sv_animcmd::frame(lua_state, 25.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "pikachu", script = "effect_attacks4", category = ACMD_GAME)]
unsafe fn pikachu_fsmash_fx(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 2.0);
            if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pikachu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
            }
        sv_animcmd::frame(lua_state, 15.0);
            if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("pikachu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pikachu_cheek_elec"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pikachu_elec_shock"), Hash40::new("top"), 0, 7, 10.5, 0, 0, 0, 1, true);
            }
        sv_animcmd::frame(lua_state, 29.0);
            if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("pikachu_cheek_elec"), false, true);
            }
        /*Active 15-16/17-19/20-29 → 15-16/17-19/20-24 (30.0 → 25.0)*/ sv_animcmd::frame(lua_state, 25.0);
            if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("pikachu_elec_shock"), false, true);
            }
        /*Active 15-16/17-19/20-29 → 15-16/17-19/20-24 (37.0 → 32.0)*/  sv_animcmd::frame(lua_state, 32.0);
            if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("pikachu_cheek2"), false, true);
            }
}

#[acmd_script( agent = "pikachu", script = "game_specials", category = ACMD_GAME)]
unsafe fn pikachu_sspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            //notify_event_msc_cmd(0x2127e37c07, *GROUND_CLIFF_CHECK_KIND_NONE);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07));
            }
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            JostleModule::set_status(fighter.module_accessor, false);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_TRIGGER);
            
            macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 
            /*BKB 40 → 60*/ 1.0, 361, 78, 0, 60, 
            4.0, 0.0, -0.7, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
            
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_CALC_ATTACK_POWER);
            AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
            }
        sv_animcmd::frame(lua_state, 13.0);
            if macros::is_excute(fighter) {
            AttackModule::set_size(fighter.module_accessor, /*ID*/ 0, /*Size*/ 3.0);
            //notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07));
            }
        sv_animcmd::wait(lua_state, 27.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
            }
}

#[acmd_script( agent = "pikachu", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn pikachu_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 11.0);
            if macros::is_excute(fighter) {
            
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            2.5, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            1.25, 0.0, 5.0, 2.75, Some(0.0), Some(5.0), Some(10.25), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
        /*Active 11-12 → 11-13 (2.0 → 3.0)*/ sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
}

#[acmd_script( agent = "pikachu", script = "game_throwlw", category = ACMD_GAME)]
unsafe fn pikachu_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 1.0);
            if macros::is_excute(fighter) {
            macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -3.0, 3.0);
            
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 
            /*BKB, 60 → 80, Damage 5.0% → 3.8%*/ 3.8, 83, 100, 0, 80, 1.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 
            3.0, 361, 100, 0, 40, 1.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        sv_animcmd::frame(lua_state, 14.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 
            3.0, 361, 0, 100, 0, 
            5.5, 0.0, 2.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
            
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
            AttackModule::clear_all(fighter.module_accessor);
            macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
            }
        sv_animcmd::frame(lua_state, 21.0);
            if macros::is_excute(fighter) {
            let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);                
            }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        pikachu_fair, 
        pikachu_bair, 
        pikachu_fsmash, 
        pikachu_fsmash_fx, 
        pikachu_sspecial, 
        pikachu_dashgrab, 
        pikachu_dthrow
    );
}
