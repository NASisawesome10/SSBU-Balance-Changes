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
use smash::phx::Vector2f;

#[acmd_script( agent = "luigi", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn luigi_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 2.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 
            /*BKB 0 → 25*/ 11.0, 55, 100, 0, 15, 
            4.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 
            /*BKB 0 → 25*/ 11.0, 55, 100, 0, 15, 
            4.0, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
        sv_animcmd::frame(lua_state, 8.0);
            if macros::is_excute(fighter) {
            macros::ATK_POWER(fighter, 0, 7);
            macros::ATK_POWER(fighter, 1, 7);
            }
        sv_animcmd::frame(lua_state, 12.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 19.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "luigi", script = "game_speciallw", category = ACMD_GAME)]
unsafe fn luigi_dspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        /*Invincibility 1-7 → 4-10*/ sv_animcmd::frame(lua_state, 4.0);
            //execute(8);
            if macros::is_excute(fighter) {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
            }
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            /*Invincibility 1-7 → 4-10*/ macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            2.0, 367, 30, 0, 80, 
            6.0, 0.0, 9.5, 5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            2.0, 367, 30, 0, 80, 
            6.0, 0.0, 9.5, -5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            2.0, 367, 30, 0, 80, 
            6.0, 0.0, 2.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 
            0.0, 180, 100, 45, 0, 
            15.5, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            //methodlib::L2CValue::as_hash()const(0, Hash40::new("top"), 0, 8, 8);
            //AttackModule::set_vec_target_pos(fighter.module_accessor);
            AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0} as *const Vector2f, 4 as u32, false);
            //methodlib::L2CValue::as_hash()const(1, Hash40::new("top"), 0, 8, 8);
            //AttackModule::set_vec_target_pos(fighter.module_accessor);
            AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0} as *const Vector2f, 4 as u32, false);
            //methodlib::L2CValue::as_hash()const(2, Hash40::new("top"), 0, 8, 8);
            //AttackModule::set_vec_target_pos(fighter.module_accessor);
            AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0} as *const Vector2f, 4 as u32, false);
            }
        sv_animcmd::frame(lua_state, 34.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }macros::FT_MOTION_RATE(fighter, 0.6);
            sv_animcmd::frame(lua_state, 44.0);
            macros::FT_MOTION_RATE(fighter, 1);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            4.0, 70, 130, 0, 85, 
            7.5, 0.0, 11.0, -9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            4.0, 70, 130, 0, 85, 
            7.5, 0.0, 11.0, 9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            4.0, 89, 130, 0, 85, 
            6.5, 0.0, 2.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "luigi", script = "game_specialairlw", category = ACMD_GAME)]
unsafe fn luigi_dspecialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        /*Invincibility 1-7 → 4-10*/ sv_animcmd::frame(lua_state, 4.0);
            //execute(8);
            if macros::is_excute(fighter) {
                macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
                }
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            /*Invincibility 1-7 → 4-10*/ macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            2.0, 367, 30, 0, 80, 
            6.0, 0.0, 9.5, 5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            2.0, 367, 30, 0, 80, 
            6.0, 0.0, 9.5, -5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            2.0, 367, 30, 0, 80, 
            6.0, 0.0, 2.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 
            0.0, 180, 100, 45, 0, 
            15.5, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            //methodlib::L2CValue::as_hash()const(0, Hash40::new("top"), 0, 8, 8);
            //AttackModule::set_vec_target_pos(fighter.module_accessor);
            AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0} as *const Vector2f, 4 as u32, false);
            //methodlib::L2CValue::as_hash()const(1, Hash40::new("top"), 0, 8, 8);
            //AttackModule::set_vec_target_pos(fighter.module_accessor);
            AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0} as *const Vector2f, 4 as u32, false);
            //methodlib::L2CValue::as_hash()const(2, Hash40::new("top"), 0, 8, 8);
            //AttackModule::set_vec_target_pos(fighter.module_accessor);
            AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0} as *const Vector2f, 4 as u32, false);
            }
        sv_animcmd::frame(lua_state, 34.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }macros::FT_MOTION_RATE(fighter, 0.6);
        sv_animcmd::frame(lua_state, 44.0);
            macros::FT_MOTION_RATE(fighter, 1);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            4.0, 70, 130, 0, 85, 
            7.5, 0.0, 11.0, -9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            4.0, 70, 130, 0, 85, 
            7.5, 0.0, 11.0, 9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            4.0, 89, 130, 0, 85, 
            6.5, 0.0, 2.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            }
        sv_animcmd::wait(lua_state, 1.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "luigi", script = "game_throwb", category = ACMD_GAME)]
unsafe fn luigi_bthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if (ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU)) {
            //if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) == true{;
            if macros::is_excute(fighter) {
            //methodlib::L2CValue::as_hash()const(*FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_b"));
            //ArticleModule::change_motion(fighter.module_accessor);
            }/*}*/if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 
            /*KBG 65 → 68*/ 10.0, 45, 68, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 
            3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        sv_animcmd::frame(lua_state, 14.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
            macros::CHECK_FINISH_CAMERA(fighter, 21, 3);
            lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER_ADDR, 1.0);
            lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER_ADDR, smash::phx::Vector3f{x: 10.0, y: -1.0, z: 0.0});
            fighter.clear_lua_stack();
            lua_args!(fighter, 5, 1);
            sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            }
        sv_animcmd::frame(lua_state, 15.0);
            if macros::is_excute(fighter) {
            macros::REVERSE_LR(fighter);
            let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);              
            }
        }
}

#[acmd_script( agent = "luigi", script = "game_throwhi", category = ACMD_GAME)]
unsafe fn luigi_uthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if (ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU)) {
            //if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) == true{;
            if macros::is_excute(fighter) {
            //methodlib::L2CValue::as_hash()const(*FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_b"));
            //ArticleModule::change_motion(fighter.module_accessor);
            }/*}*/if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 
            /*KBG 72 → 96*/ 8.0, 90, 96, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 
            3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        sv_animcmd::frame(lua_state, 15.0);
            if macros::is_excute(fighter) {
            macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 6.0, 88, 55, 0, 95, 8.5, 0.0, 8.0, 10.0, Some(0.5), Some(1.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
            }
        sv_animcmd::frame(lua_state, 16.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 17.0);
            if macros::is_excute(fighter) {
            macros::CHECK_FINISH_CAMERA(fighter, -2, 24);
            lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER_ADDR, 1.5);
            lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER_ADDR, smash::phx::Vector3f{x: 0.0, y: 7.0, z: 0.0});
            }
        sv_animcmd::frame(lua_state, 18.0);
            if macros::is_excute(fighter) {
            let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);  
            }
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        luigi_uair, 
        luigi_dspecial, 
        luigi_dspecialair, 
        luigi_bthrow, 
        luigi_uthrow
    );
}
