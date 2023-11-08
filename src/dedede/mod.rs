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

#[acmd_script( agent = "dedede", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn dedede_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*BKB 60 → 65, KBG 75 → 90*/ 10.0, 35, 90, 0, 65, 
            7.0, 0.0, 6.5, 12.0, Some(0.0), Some(6.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*BKB 60 → 65, KBG 75 → 90*/ 6.0, 35, 90, 0, 65, 
            5.0, 0.0, 5.0, 12.0, Some(0.0), Some(5.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            }
        sv_animcmd::wait(lua_state, 4.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "dedede", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn dedede_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        sv_animcmd::frame(lua_state, 13.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 
            /*KBG 98 → 108*/ 12.0, 361, 108, 0, 15, 
            8.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 
            /*KBG 98 → 108*/ 12.0, 361, 108, 0, 15, 
            5.0, -12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            }
        sv_animcmd::frame(lua_state, 16.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 40.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "dedede", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn dedede_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        sv_animcmd::frame(lua_state, 17.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*KBG 88 → 92*/ 16.0, 361, 92, 0, 30, 
            6.5, 0.0, 10.0, -18.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*KBG 88 → 92*/ 16.0, 361, 92, 0, 30, 
            6.5, 0.0, 10.0, -12.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            16.0, 361, 88, 0, 30, 
            6.5, 0.0, 10.0, -6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            }
        sv_animcmd::frame(lua_state, 20.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 33.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "dedede", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn dedede_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        /*Active 10/12/14/16/18/20/22/24-25 → 8/10/12/14/16/18/20/22-24 (10.0 → 8.0)*/ sv_animcmd::frame(lua_state, 8.0);
            for _ in 0..7{;
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 
            1.0, 367, 100, 20, 0, 
            6.0, 17.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 
            1.0, 367, 100, 20, 0, 
            6.0, 17.0, 0.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 
            1.0, 95, 100, 40, 0, 
            4.0, 9.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 
            1.0, 95, 100, 40, 0, 
            4.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::wait(lua_state, 1.0);
            }if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 
            5.0, 60, 167, 0, 55, 
            /*Size 6.5 → 7.5*/ 7.5, 17.0, 0.0, -3.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 
            5.0, 60, 167, 0, 55, 
            /*Size 6.5 → 7.5*/ 7.5, 17.0, 0.0, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 
            5.0, 60, 167, 0, 55, 
            /*Size 5.0 → 6.0*/ 6.0, 9.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 
            5.0, 60, 167, 0, 55, 
            /*Size 5.0 → 6.0*/ 6.0, 3.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 42.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "dedede", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn dedede_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 32.0);
            if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 6.0, 7.0);
            }
        sv_animcmd::frame(lua_state, 34.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        sv_animcmd::frame(lua_state, 40.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 
            16.0, 45, 55, 0, 85, 
            /*Size 4.5 → 5.5*/ 5.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            // methodlib::L2CValue::as_hash()const(0, Hash40::new("top"), 16, 2, 2);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("top"), 16, false, 2);
            //AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), smash::phx::Vector2f{x: 16.0, y: 2.0}, 7.0 as u32, false);
            // methodlib::L2CValue::as_hash()const(1, Hash40::new("top"), 16, 2, 2);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("top"), 16, false, 2);
            //AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), smash::phx::Vector2f{x: 16.0, y: 2.0}, 7.0 as u32, false);
        
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 
            /*Damage 0.0% → 8.0%*/ 8.0, 0, 0, 0, 0, 
            3.5, -6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 
            /*Damage 0.0% → 8.0%*/ 8.0, 0, 0, 0, 0, 
            3.5, -12.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
            
            
        }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 
            18.5, 45, 85, 0, 50, 
            3.5, 6.0, -3.0, 0.0, Some(3.0), Some(-5.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 
            25.0, 361, 74, 0, 70, 
            /*Size 6.0 → 8.0*/ 8.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            }
        sv_animcmd::frame(lua_state, 44.0);
            if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            11.0, 60, 30, 0, 100, 8.0, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "dedede", script = "game_specialnstart", category = ACMD_GAME)]
unsafe fn dedede_nspecialstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 2.0);
            macros::FT_MOTION_RATE(fighter, 0.8);
    sv_animcmd::frame(lua_state, 17.0);
            /*Intangibility on his head.*/ macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);

            macros::FT_MOTION_RATE(fighter, 1.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 9.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(7.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 
                5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            //search!(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 13.0, 5.5, Some(0.0), Some(3.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            }if (WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY){;
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 7.5, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(7.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 
                5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            //search!(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            }}
}

#[acmd_script( agent = "dedede", script = "game_specialnloop", category = ACMD_GAME)]
unsafe fn dedede_nspecialloop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            /*Intangibility on his head.*/ macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);

            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            5.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(12.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            0.0, 170, 100, 30, 0, 
            6.0, 0.0, 9.0, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            0.0, 170, 100, 10, 0, 
            7.0, 0.0, 6.0, 23.0, Some(0.0), Some(12.0), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            0.0, 0, 0, 0, 0, 
            9.0, 0.0, 8.5, 10.0, Some(0.0), Some(8.5), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            //search!(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 13.0, 5.5, Some(0.0), Some(3.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            
            //search!(fighter, 1, 0, Hash40::new("top"), 9.5, 0.0, 8.5, 11.0, Some(0.0), Some(8.5), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            }if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            5.0, 0.0, 5.5, 5.0, Some(0.0), Some(5.5), Some(12.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            0.0, 170, 100, 30, 0, 
            6.0, 0.0, 6.5, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            0.0, 170, 100, 10, 0, 
            7.0, 0.0, 3.5, 23.0, Some(0.0), Some(9.5), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            //search!(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            
            //search!(fighter, 1, 0, Hash40::new("top"), 9.5, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            }}
}

#[acmd_script( agent = "dedede", script = "game_specialairnstart", category = ACMD_GAME)]
unsafe fn dedede_nspecialstartair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            /*Intangibility on his head.*/ macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);

            macros::FT_MOTION_RATE(fighter, 1.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 9.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(7.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 
                5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            //search!(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 13.0, 5.5, Some(0.0), Some(3.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            }if (WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY){;
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 7.5, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(7.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 
                5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            //search!(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            }}
}

#[acmd_script( agent = "dedede", script = "game_specialairnloop", category = ACMD_GAME)]
unsafe fn dedede_nspecialloopair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            /*Intangibility on his head.*/ macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);

            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            5.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(9.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            0.0, 170, 100, 30, 0, 
            6.0, 0.0, 9.0, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            0.0, 170, 100, 10, 0, 
            7.0, 0.0, 6.0, 23.0, Some(0.0), Some(12.0), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            0.0, 0, 0, 0, 0, 
            9.0, 0.0, 8.5, 10.0, Some(0.0), Some(8.5), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            //search!(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 13.0, 5.5, Some(0.0), Some(3.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            
            //search!(fighter, 1, 0, Hash40::new("top"), 9.5, 0.0, 8.5, 11.0, Some(0.0), Some(8.5), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            }if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            5.0, 0.0, 5.5, 5.0, Some(0.0), Some(5.5), Some(9.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            0.0, 170, 100, 30, 0, 
            6.0, 0.0, 6.5, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            0.0, 170, 100, 10, 0, 
            7.0, 0.0, 3.5, 23.0, Some(0.0), Some(9.5), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            //search!(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            
            //search!(fighter, 1, 0, Hash40::new("top"), 9.5, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            }}
}

#[acmd_script( agent = "dedede", script = "game_catch", category = ACMD_GAME)]
unsafe fn dedede_standinggrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 8.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            /*Size 4.5 → 5.0*/ 5.0, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(12.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            /*Size 2.25 → 2.5*/ 2.5, 0.0, 5.0, 1.75, Some(0.0), Some(5.0), Some(14.75), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
        sv_animcmd::frame(lua_state, 12.0);
            macros::FT_MOTION_RATE(fighter, 1.12);
}

#[acmd_script( agent = "dedede", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn dedede_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 11.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            /*Size 3.6 → 4.0*/ 4.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(14.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            /*Size 1.8 → 2.0*/ 2.0, 0.0, 7.0, 2.2, Some(0.0), Some(7.0), Some(16.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
        sv_animcmd::frame(lua_state, 17.0);
            macros::FT_MOTION_RATE(fighter, 1.03);
}

#[acmd_script( agent = "dedede", script = "game_catchpivot", category = ACMD_GAME)]
unsafe fn dedede_pivotgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 11.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 12.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            /*Size 4.5 → 5.0*/ 5.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(-19.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            /*Size 2.25 → 2.5*/ 2.5, 0.0, 6.0, -1.75, Some(0.0), Some(6.0), Some(-21.55), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        dedede_dtilt, 
        dedede_fair, 
        dedede_bair, 
        dedede_uair, 
        dedede_fsmash, 
        dedede_nspecialstart, 
        dedede_nspecialloop, 
        dedede_nspecialstartair, 
        dedede_nspecialloopair, 
        dedede_standinggrab, 
        dedede_dashgrab, 
        dedede_pivotgrab
    );
}
