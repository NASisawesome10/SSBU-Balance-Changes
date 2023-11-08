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

#[acmd_script( agent = "koopajr", script = "game_attack100end", category = ACMD_GAME)]
unsafe fn koopajr_rapidjabfinisher(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 1.0);
            if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5.0, 5.0);
            }
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            3.0, 361, 170, 0, 60, 
            5.5, 0.0, 6.5, 13.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            3.0, 361, 170, 0, 60, 
            /*Size 5.5 → 6.5*/ 6.5, 0.0, 6.5, 18.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            3.0, 361, 170, 0, 60, 
            /*Size 5.5 → 6.5*/ 6.5, 0.0, 6.5, 22.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            } 
        /*Active 6-7 → 6-8, (2.0 → 3.0)*/ sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attacks3", category = ACMD_GAME)]
unsafe fn koopajr_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 
            /*Angle 361° → 30°*/ 6.0, 30, 100, 0, 32, 
            2.3, 0.0, 0.0, 0.0, Some(4.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 
            /*Angle 361° → 30°*/ 8.0, 30, 100, 0, 32, 
            2.8, 8.5, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attacks3hi", category = ACMD_GAME)]
unsafe fn koopajr_ftilthi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 
            /*Angle 361° → 30°*/ 6.0, 30, 70, 0, 72, 
            2.3, 0.0, 0.0, 0.0, Some(4.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 
            /*Angle 361° → 30°*/ 8.0, 30, 70, 0, 72, 
            2.8, 8.5, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attacks3lw", category = ACMD_GAME)]
unsafe fn koopajr_ftiltlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 
            /*Angle 361° → 30°*/ 6.0, 30, 70, 0, 72, 
            2.3, 0.0, 0.0, 0.0, Some(4.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 
            /*Angle 361° → 30°*/ 8.0, 30, 70, 0, 72, 
            2.8, 8.5, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn koopajr_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 4.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clowntongue2"), 
            2.0, 94, 11, 0, 17, 
            3.0, 9.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clowntongue2"), 
            2.0, 45, 11, 0, 17, 
            2.8, 3.0, 1.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            
            // ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.2);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 4.0, /*Unk*/ false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 4.0, /*Unk*/ false);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);            
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 12.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clowntongue2"), 
            /*Angle 94° → 174°*/ 2.0, 174, 11, 0, 17, 
            3.0, 9.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clowntongue2"), 
            2.0, 55, 11, 0, 17, 
            2.8, 3.0, 1.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            
            // ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.2);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 4.0, /*Unk*/ false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 4.0, /*Unk*/ false);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 23.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clowntongue2"), 
            6.0, 40, 150, 0, 20, 
            /*Size 4.0 → 5.0*/ 4.0, 9.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clowntongue2"), 
            6.0, 40, 150, 0, 20, 
            /*Size 2.8 → 4.0*/ 2.8, 3.0, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn koopajr_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 8.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            2.0, 30, 10, 0, 45, 
            4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            2.0, 34, 10, 0, 45, 
            4.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            /*Angle 65° → 165°*/ macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            2.0, 165, 10, 0, 20, 
            4.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 5.0);
            if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            2.0, 32, 10, 0, 35, 
            4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            /*Angle 65° → 165°*/ macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            2.0, 165, 10, 0, 20, 
            4.0, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 8.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 25.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            4.0, 50, 70, 0, 100, 
            5.0, 0.0, 8.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clowntongue3"), 
            4.0, 50, 70, 0, 100, 
            /*Z 0.0 → 1.0*/ 6.0, 0.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn koopajr_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        /*Active 7-8(9-13)(14-19) → 6-8(9-13)(14-19) (7.0 → 6.0)*/ sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownarmr1"), 
            6.5, 361, 83, 0, 50, 
            4.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 
            6.5, 361, 83, 0, 50, 
            4.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("clownarmr1"), 
            8.0, 361, 83, 0, 50, 
            6.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("clownarml1"), 
            8.0, 361, 83, 0, 50, 
            6.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownarmr1"), 
            5.5, 361, 81, 0, 40, 
            3.5, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 
            5.5, 361, 81, 0, 40, 
            3.5, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("clownarmr1"), 
            7.0, 361, 81, 0, 40, 
            5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("clownarml1"), 
            7.0, 361, 81, 0, 40, 
            5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        sv_animcmd::wait(lua_state, 5.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownarmr1"), 
            3.5, 361, 80, 0, 30, 
            3.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 
            3.5, 361, 80, 0, 30, 
            3.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("clownarmr1"), 
            5.0, 361, 80, 0, 30, 
            5.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("clownarml1"), 
            5.0, 361, 80, 0, 30, 
            5.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        sv_animcmd::wait(lua_state, 6.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 42.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn koopajr_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 3.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        /*Active 10-13(14-17)(18-23)/2 → 8-13(14-17)(18-23)/2 (10.0 → 8.0)*/ sv_animcmd::frame(lua_state, 8.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 
            11.0, 45, 93, 0, 33, 
            6.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 
            9.0, 45, 93, 0, 33, 
            3.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::frame(lua_state, 14.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 
            9.0, 45, 93, 0, 33, 
            5.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 
            7.0, 45, 93, 0, 33, 
            3.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::frame(lua_state, 18.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 
            7.0, 45, 93, 0, 33, 
            5.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 
            5.0, 45, 93, 0, 33, 
            2.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::frame(lua_state, 24.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 34.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn koopajr_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 2.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            JostleModule::set_status(fighter.module_accessor, false);
            }
        sv_animcmd::frame(lua_state, 15.0);
            macros::FT_MOTION_RATE(fighter, 0.75);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*BKB 30 → 70, Angle 94° → 367°*/1.5, 367, 10, 0, 70, 
            /*Size 4.0 → 5.5, Y -2.0 → -1.0*/ 5.5, 0.0, -1.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::frame(lua_state, 47.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 
            2.5, 64, 124, 0, 70, 
            4.0, 0.0, 2.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 
            2.5, 64, 124, 0, 70, 
            4.5, 0.0, -3.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            JostleModule::set_status(fighter.module_accessor, true);
            }
        sv_animcmd::frame(lua_state, 68.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "koopajr", script = "game_landingairlw", category = ACMD_GAME)]
unsafe fn koopajr_dairlanding(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 1.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_LANDING_ATTACK_AIR_SQUAT);
            JostleModule::set_status(fighter.module_accessor, true);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*Angle 50° → 65°*/ 2.0, 65, 180, 0, 40, 
            5.0, 0.0, 4.0, 2.5, Some(0.0), Some(4.0), Some(-2.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn koopajr_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        sv_animcmd::frame(lua_state, 18.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.0, 180, 0, 0, 20, 
            4.5, 0.0, 8.0, 16.5, Some(0.0), Some(12.5), Some(16.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            1.0, 361, 0, 0, 20, 
            2.5, 0.0, 7.0, 9.7, Some(0.0), Some(12.0), Some(9.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            /*There is now a hitbox above and in front of the current ones*/ macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            0.5, 260, 0, 0, 30, 
            2.5, 0.0, 19.0, 16.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            /*There is now a hitbox below and in front of the current ones*/ macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            0.5, 100, 0, 0, 30, 
            2.5, 0.0, 1.0, 16.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 15.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 35.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            11.0, 46, 136, 0, 40, 
            /*Size 6.0 → 7.0*/ 7.0, 0.0, 9.0, 17.0, Some(0.0), Some(12.0), Some(17.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attacks4hi", category = ACMD_GAME)]
unsafe fn koopajr_fsmashhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        sv_animcmd::frame(lua_state, 18.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.0, 180, 0, 0, 20, 
            4.5, 0.0, 12.0, 15.0, Some(0.0), Some(16.5), Some(14.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            1.0, 361, 0, 0, 20, 
            2.5, 0.0, 10.0, 8.3, Some(0.0), Some(15.0), Some(7.8), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            /*There is now a hitbox above and in front of the current ones*/ macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            0.5, 260, 0, 0, 30, 
            2.5, 0.0, 22.0, 15.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            /*There is now a hitbox below and in front of the current ones*/ macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            0.5, 100, 0, 0, 30, 
            2.5, 0.0, 6.0, 17.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 15.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 35.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            11.0, 46, 136, 0, 40, 
            /*Size 6.0 → 7.0*/ 7.0, 0.0, 13.0, 16.0, Some(0.0), Some(16.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attacks4lw", category = ACMD_GAME)]
unsafe fn koopajr_fsmashlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        sv_animcmd::frame(lua_state, 18.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.0, 180, 0, 0, 20, 
            4.5, 0.0, 3.5, 16.0, Some(0.0), Some(7.5), Some(17.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            1.0, 361, 0, 0, 20, 
            2.5, 0.0, 4.5, 9.1, Some(0.0), Some(9.0), Some(10.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            /*There is now a hitbox above and in front of the current ones*/ macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            0.5, 260, 0, 0, 30, 
            2.5, 0.0, 13.0, 16.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            /*There is now a hitbox below and in front of the current ones*/ macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            0.5, 100, 0, 0, 30, 
            2.5, 0.0, -1.5, 15.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 15.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 35.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            11.0, 46, 136, 0, 40, 
            /*Size 6.0 → 7.0*/ 7.0, 0.0, 4.5, 16.5, Some(0.0), Some(6.5), Some(17.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn koopajr_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 1.0);
            if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 6.5);
            }
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 
            1.0, 102, 100, 80, 0, 
            4.5, 0.0, 10.0, -5.0, Some(0.0), Some(10.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.7, 105, 100, 18, 0, 
            3.2, 0.0, 18.0, -6.5, Some(0.0), Some(16.0), Some(7.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            1.7, 175, 100, 20, 0, 
            3.2, 0.0, 21.0, -6.5, Some(0.0), Some(19.0), Some(7.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            1.7, 140, 100, 50, 30, 
            4.7, 0.0, 18.0, -6.5, Some(0.0), Some(16.0), Some(7.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            
            /*There is now a hitbox below all the others.*/ macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 
            1.7, 102, 100, 80, 0, 
            2.3, 0.0, 10.0, -5.0, Some(0.0), Some(10.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 9.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 
            1.4, 120, 100, 35, 0, 5.1, 0.0, 19.0, -6.5, Some(0.0), Some(17.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 22.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            6.0, 85, 175, 0, 38, 
            6.5, 0.0, 23.0, -5.0, Some(0.0), Some(21.0), Some(5.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "koopajr", script = "game_specialnstart", category = ACMD_GAME)]
unsafe fn koopajr_nspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 1.0);
        /*0.84 → 1.0*/ macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "koopajr", script = "game_specials", category = ACMD_GAME)]
unsafe fn koopajr_sspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 8.0);
            JostleModule::set_status(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownhip"), 
            4.0, 80, 95, 0, 60, 
            /*Size 3.6 → 4.0, Z 0.0 → 1.0*/ 4.0, 0.0, 1.5, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownhip"), 
            4.0, 80, 95, 0, 60, 
            /*Size 1.5 → 2.5, Z 0.0 → 1.0*/ 2.5, 0.0, 3.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
}

#[acmd_script( agent = "koopajr", script = "game_specialairs", category = ACMD_GAME)]
unsafe fn koopajr_sspecialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 8.0);
            JostleModule::set_status(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownhip"), 
            2.0, 55, 120, 0, 65, 
            /*Size 3.6 → 4.0, Z 0.0 → 1.0*/ 4.0, 0.0, 1.5, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownhip"), 
            2.0, 55, 120, 0, 65, 
            /*Size 1.5 → 2.5, Z 0.0 → 1.0*/ 2.5, 0.0, 3.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
}

#[acmd_script( agent = "koopajr", script = "game_specialslanding", category = ACMD_GAME)]
unsafe fn koopajr_sspeciallanding(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 8.0);
            JostleModule::set_status(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("clownhip"), 
            4.0, 80, 95, 0, 60, 
            /*Size 3.6 → 4.0, Z 0.0 → 1.0*/ 4.0, 0.0, 1.5, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("clownhip"), 
            4.0, 80, 95, 0, 60, 
            /*Size 1.5 → 2.5, Z 0.0 → 1.0*/ 2.5, 0.0, 3.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
}

#[acmd_script( agent = "koopajr", script = "game_specialsspin", category = ACMD_GAME)]
unsafe fn koopajr_sspecialspin(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            JostleModule::set_status(fighter.module_accessor, false);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
            }
        sv_animcmd::frame(lua_state, 3.0);
            //execute(3);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
            macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 
            10.0, 361, 75, 0, 60, 
            5.2, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::frame(lua_state, 15.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        /*He now has a weaker hitbox for his second spin.*/ sv_animcmd::frame(lua_state, 25.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
            macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 
            10.0, 361, 45, 0, 50, 
            5.2, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        /*He now has a weaker hitbox for his second spin. (15.0 → 33.0)*/  sv_animcmd::frame(lua_state, 33.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 35.0);
            if macros::is_excute(fighter) {
            JostleModule::set_status(fighter.module_accessor, true);
            }
}

#[acmd_script( agent = "koopajr", script = "game_specialairsspin", category = ACMD_GAME)]
unsafe fn koopajr_sspecialspinair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            JostleModule::set_status(fighter.module_accessor, false);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
            }
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
            macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 
            8.0, 361, 80, 0, 60, 
            5.2, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::wait(lua_state, 8.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_NORMAL_GRAVITY);
            }
        sv_animcmd::frame(lua_state, 15.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        /*He now has a weaker hitbox for his second spin.*/ sv_animcmd::frame(lua_state, 25.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
            macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 
            8.0, 361, 50, 0, 50, 
            5.2, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        /*He now has a weaker hitbox for his second spin. (15.0 → 33.0)*/ sv_animcmd::frame(lua_state, 33.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 40.0);
            if macros::is_excute(fighter) {
            JostleModule::set_status(fighter.module_accessor, true);
            }
}

// Return clown kart after being hit
//#[fighter_frame_callback]
#[fighter_frame( agent = FIGHTER_KIND_KOOPAJR )]
fn koopajr_clownkartreturn(fighter: &mut L2CFighterCommon) {
    unsafe{
        if(WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT)){
            if(   StatusModule::status_kind(fighter.module_accessor) == smash::lib::lua_const::FIGHTER_STATUS_KIND_DAMAGE
               || StatusModule::status_kind(fighter.module_accessor) == smash::lib::lua_const::FIGHTER_STATUS_KIND_DAMAGE_AIR
               || StatusModule::status_kind(fighter.module_accessor) == smash::lib::lua_const::FIGHTER_STATUS_KIND_DAMAGE_FLY 
               || StatusModule::status_kind(fighter.module_accessor) == smash::lib::lua_const::FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
               || StatusModule::status_kind(fighter.module_accessor) == smash::lib::lua_const::FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
               || StatusModule::status_kind(fighter.module_accessor) == smash::lib::lua_const::FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
               || StatusModule::status_kind(fighter.module_accessor) == smash::lib::lua_const::FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
               || StatusModule::status_kind(fighter.module_accessor) == smash::lib::lua_const::FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
               || StatusModule::status_kind(fighter.module_accessor) == smash::lib::lua_const::FIGHTER_STATUS_KIND_DAMAGE_FALL){
                
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT);
               //  WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_STATUS_SPECIAL_HI_SHOOT_FLAG_ACTION);
                // WorkModule::on_flag(fighter.module_accessor, *WEAPON_KIND_KOOPAJR_KART);
                // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_KART);
            }
        }
    }
}

#[acmd_script( agent = "koopajr_remainclown", script = "game_specialhiclownfall", category = ACMD_GAME)]
unsafe fn koopajr_uspecialclownfall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            5.0, 90, 100, 80, 0, 
            /*Size 6.0 → 7.5*/ 7.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            5.0, 86, 35, 0, 20, 
            /*Size 6.0 → 7.5*/ 7.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::frame(lua_state, 16.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            5.0, 90, 100, 55, 0, 
            /*Size 6.0 → 7.5*/ 7.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
}

#[acmd_script( agent = "koopajr_remainclown", script = "game_specialairhiclownfall", category = ACMD_GAME)]
unsafe fn koopajr_uspecialclownfallair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            5.0, 90, 100, 80, 0, 
            /*Size 6.0 → 7.5*/ 7.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            5.0, 86, 35, 0, 20, 
            /*Size 6.0 → 7.5*/ 7.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        sv_animcmd::frame(lua_state, 16.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            5.0, 90, 100, 55, 0, 
            /*Size 6.0 → 7.5*/ 7.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
}

#[acmd_script( agent = "koopajr_remainclown", script = "game_specialairhiburst", category = ACMD_GAME)]
unsafe fn koopajr_uspecialburst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            VisibilityModule::set_whole(fighter.module_accessor, false);
            /*Size 14.0 → 16.0*/ macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            13.0, 55, 70, 0, 85, 
            16.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
            macros::AREA_WIND_2ND_RAD_arg9(fighter, 0, 3, 0.05, 300, 1, 0, 6, 34, 80);
            // methodlib::L2CValue::as_hash()const(Hash40::new("rbkind_explosion"), 0, false);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
            }
        sv_animcmd::frame(lua_state, 2.0);
            if macros::is_excute(fighter) {
            sv_battle_object::notify_event_msc_cmd(0x199c462b5d);
            }
}

#[acmd_script( agent = "koopajr", script = "game_throwb", category = ACMD_GAME)]
unsafe fn koopajr_bthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            /*KBG 70 → 75, Angle 45° → 40°*/ macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 40, 75, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        sv_animcmd::frame(lua_state, 23.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
            macros::CHECK_FINISH_CAMERA(fighter, 20, 10);
            lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER_ADDR, 1.5);
            lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER_ADDR, smash::phx::Vector3f{x: 10.0, y: 2.0, z: 0.0});
            }
        sv_animcmd::frame(lua_state, 24.0);
            if macros::is_excute(fighter) {
            macros::REVERSE_LR(fighter);
            let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);             
            }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        koopajr_rapidjabfinisher, 
        koopajr_ftilt, 
        koopajr_ftilthi, 
        koopajr_ftiltlw, 
        koopajr_dtilt, 
        koopajr_dashattack, 
        koopajr_nair, 
        koopajr_fair, 
        koopajr_dair, 
        koopajr_dairlanding, 
        koopajr_fsmash, 
        koopajr_fsmashhi, 
        koopajr_fsmashlw, 
        koopajr_usmash, 
        koopajr_nspecial, 
        koopajr_sspecial, 
        koopajr_sspecialair, 
        koopajr_sspeciallanding, 
        koopajr_sspecialspin, 
        koopajr_sspecialspinair, 
        koopajr_uspecialclownfall, 
        koopajr_uspecialclownfallair, 
        koopajr_uspecialburst, 
        koopajr_bthrow
    );
}

pub fn install_agent_code() {
    smashline::install_agent_frames!(
        // koopajr_clownkartreturn
    );
}