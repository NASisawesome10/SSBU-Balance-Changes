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

#[acmd_script( agent = "pichu", script = "game_attacks3", category = ACMD_GAME)]
unsafe fn pichu_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            /*Recoil Damage 1.0% → 0.4%*/ macros::FT_ADD_DAMAGE(fighter, 0.4);
            macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 
            8.0, 361, 130, 0, 5, 
            3.5, 0.0, 0.0, 0.0, Some(0.0), Some(-4.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            }
        sv_animcmd::frame(lua_state, 9.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 
            8.0, 361, 130, 0, 5, 
            3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-3.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            }
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
            }
        sv_animcmd::frame(lua_state, 13.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "pichu", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn pichu_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            /*Recoil Damage 1.5% → 0.6%*/ macros::FT_ADD_DAMAGE(fighter, 0.6);
            }
            for _ in 0..3{;
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 
            3.5, 366, 100, 20, 0, 
            6.8, 2.2, 0.5, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 
            3.5, 366, 100, 20, 0, 
            5.6, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::wait(lua_state, 1.0);
            }
        sv_animcmd::frame(lua_state, 21.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("neck"),
            3.5, 45, 140, 0, 55, 
            6.8, 2.2, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 
            3.5, 45, 140, 0, 55, 
            5.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
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

#[acmd_script( agent = "pichu", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn pichu_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 5.0);
            if macros::is_excute(fighter) {
            /*Recoil Damage 1.5% → 0.6%*/ macros::FT_ADD_DAMAGE(fighter, 0.6);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            2.0, 367, 100, 50, 0, 
            4.0, 0.0, 1.8, -11.5, Some(0.0), Some(1.8), Some(0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            }
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 11.0);
            for _ in 0..3{;
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            2.0, 367, 100, 50, 0, 
            3.5, 0.0, 2.5, -10.5, Some(0.0), Some(2.5), Some(-0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            }
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::wait(lua_state, 2.0);
            }
        sv_animcmd::frame(lua_state, 26.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            2.0, 367, 100, 50, 0, 
            3.5, 0.0, 2.5, -10.5, Some(0.0), Some(2.5), Some(-0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            }
        sv_animcmd::frame(lua_state, 28.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 30.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            2.5, 42, 160, 0, 70, 
            5.0, 0.0, 3.0, -10.5, Some(0.0), Some(3.0), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            }
        sv_animcmd::frame(lua_state, 32.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 38.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "pichu", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn pichu_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        sv_animcmd::frame(lua_state, 14.0);
            if macros::is_excute(fighter) {
            /*Recoil Damage 1.5% → 0.6%*/ macros::FT_ADD_DAMAGE(fighter, 0.6);
            macros::HIT_NODE(fighter, Hash40::new("mimir1"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("mimil1"), *HIT_STATUS_XLU);
            macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 
            13.0, 270, 86, 0, 16, 
            5.5, 4.5, -1.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            }
        sv_animcmd::frame(lua_state, 18.0);
            if macros::is_excute(fighter) {
            HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 
            12.0, 361, 100, 0, 20, 
            5.4, 4.5, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 
            12.0, 361, 100, 0, 20, 
            3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            }
        sv_animcmd::frame(lua_state, 27.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 32.0);
            if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "pichu", script = "game_specialn", category = ACMD_GAME)]
unsafe fn pichu_nspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 18.0);
            if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, 0);
            }
            if macros::is_excute(fighter) {
            /*Recoil Damage 0.7% → 0.5%*/ macros::FT_ADD_DAMAGE(fighter, 0.5);
            }
}

#[acmd_script( agent = "pichu", script = "game_specials", category = ACMD_GAME)]
unsafe fn pichu_sspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            //notify_event_msc_cmd(0x2127e37c07, *GROUND_CLIFF_CHECK_KIND_NONE);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07));
            }
        sv_animcmd::frame(lua_state, 4.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_TRIGGER);
            macros::FT_ADD_DAMAGE(fighter, 1.5);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*BKB 15 → 20*/ 1.0, 361, 90, 0, 20, 
            3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_CALC_ATTACK_POWER);
            AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
            }
        sv_animcmd::frame(lua_state, 11.0);
            if macros::is_excute(fighter) {
            //notify_event_msc_cmd(0x2127e37c07, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07));
            }
        sv_animcmd::frame(lua_state, 24.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
            }
}

#[acmd_script( agent = "pichu", script = "game_specialhi2", category = ACMD_GAME)]
unsafe fn pichu_uspecial2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            /*Recoil Damage 1.0% → 0.5%*/ macros::FT_ADD_DAMAGE(fighter, 0.5);
            JostleModule::set_status(fighter.module_accessor, false);
            }
}

#[acmd_script( agent = "pichu", script = "game_speciallwhit", category = ACMD_GAME)]
unsafe fn pichu_dspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
            if macros::is_excute(fighter) {
            /*Recoil Damage 3.5% → 1.5%*/ macros::FT_ADD_DAMAGE(fighter, 1.5);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            14.0, 361, 65, 0, 90, 
            11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            }
        sv_animcmd::wait(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "pichu", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn pichu_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 8.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            1.3, 0.0, 6.6, 2.7, Some(0.0), Some(6.6), Some(11.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
        /*Active 8-9 → 8-10 (2.0 → 3.0)*/ sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        pichu_ftilt, 
        pichu_fair, 
        pichu_bair, 
        pichu_dair, 
        pichu_nspecial, 
        pichu_sspecial, 
        pichu_uspecial2, 
        pichu_dspecial, 
        pichu_dashgrab
    );
}
