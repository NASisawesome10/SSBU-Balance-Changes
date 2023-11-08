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

#[acmd_script( agent = "chrom", script = "game_specials4hi", category = ACMD_GAME)]
unsafe fn chrom_sspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            /*BKB 80 → 70*/ 6.46, 79, 37, 0, 70, 
            3.5, 0.0, 6.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 1.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            /*BKB 65 → 55*/ 6.46, 87, 37, 0, 55, 
            3.5, 0.0, 6.0, 14.0, Some(0.0), Some(21.0), Some(14.0), 1.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            /*BKB 80 → 70*/ 6.46, 79, 37, 0, 70, 
            3.5, 0.0, 21.0, 10.0, Some(0.0), Some(24.0), Some(3.5), 1.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            /*BKB 65 → 55*/ 6.46, 87, 37, 0, 55, 
            3.5, 0.0, 22.5, 14.0, Some(0.0), Some(26.5), Some(4.0), 1.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            }
        sv_animcmd::frame(lua_state, 11.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

#[acmd_script( agent = "chrom", script = "game_specialhi2", category = ACMD_GAME)]
unsafe fn chrom_uspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 1.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            6.0, 91, 100, 155, 0, 
            4.8, 0.0, 5.0, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            6.0, 89, 100, 155, 0, 
            4.8, 0.0, 5.0, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            6.0, 90, 100, 140, 0, 
            4.8, 0.0, 12.5, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            6.0, 91, 100, 140, 0, 
            4.8, 0.0, 12.5, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            }
        /*All hitboxes will move up about 2 meters in game on frame 11, then go away.*/ sv_animcmd::frame(lua_state, 2.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            6.0, 91, 100, 155, 0, 
            4.8, 0.0, 10.0, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            6.0, 89, 100, 155, 0, 
            4.8, 0.0, 10.0, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            6.0, 90, 100, 140, 0, 
            4.8, 0.0, 17.5, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            6.0, 91, 100, 140, 0, 
            4.8, 0.0, 17.5, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            }
        sv_animcmd::frame(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 22.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            }
        sv_animcmd::frame(lua_state, 23.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTROL);
            WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
            WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
            }
        sv_animcmd::frame(lua_state, 27.0);
            macros::FT_MOTION_RATE(fighter, 0.5);
        sv_animcmd::frame(lua_state, 31.0);
            macros::FT_MOTION_RATE(fighter, 1);
        sv_animcmd::frame(lua_state, 32.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.5, 366, 100, 125, 0, 
            9.0, 0.0, 16.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        sv_animcmd::frame(lua_state, 41.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 44.0);
            if macros::is_excute(fighter) {
            //notify_event_msc_cmd(0x2127e37c07, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07));
            }
}

#[acmd_script( agent = "chrom", script = "game_specialairhi2", category = ACMD_GAME)]
unsafe fn chrom_uspecialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 1.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            6.0, 91, 100, 155, 0, 
            4.8, 0.0, 5.0, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            6.0, 89, 100, 155, 0, 
            4.8, 0.0, 5.0, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            6.0, 90, 100, 140, 0, 
            4.8, 0.0, 12.5, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            6.0, 91, 100, 140, 0, 
            4.8, 0.0, 12.5, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            }
        /*All hitboxes will move up about 2 meters in game on frame 11, then go away.*/ sv_animcmd::frame(lua_state, 2.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            6.0, 91, 100, 155, 0, 
            4.8, 0.0, 10.0, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 
            6.0, 89, 100, 155, 0, 
            4.8, 0.0, 10.0, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 
            6.0, 90, 100, 140, 0, 
            4.8, 0.0, 17.5, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 
            6.0, 91, 100, 140, 0, 
            4.8, 0.0, 17.5, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            }
        sv_animcmd::frame(lua_state, 3.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 22.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            }
        sv_animcmd::frame(lua_state, 23.0);
            if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTROL);
            WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
            WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
            }
        sv_animcmd::frame(lua_state, 27.0);
            macros::FT_MOTION_RATE(fighter, 0.5);
        sv_animcmd::frame(lua_state, 31.0);
            macros::FT_MOTION_RATE(fighter, 1);
        sv_animcmd::frame(lua_state, 32.0);
            if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 
            1.5, 366, 100, 125, 0, 
            9.0, 0.0, 16.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        sv_animcmd::frame(lua_state, 41.0);
            if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            }
        sv_animcmd::frame(lua_state, 44.0);
            if macros::is_excute(fighter) {
            //notify_event_msc_cmd(0x2127e37c07, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07));
            }
}

#[acmd_script( agent = "chrom", script = "game_catch", category = ACMD_GAME)]
unsafe fn chrom_standinggrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 6.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 7.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            /*Z2 = 7.4 → 8.4*/ 3.3, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(8.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            /*Z2 = 9.05 → 10.05*/ 1.65, 0.0, 8.0, 2.35, Some(0.0), Some(8.0), Some(10.05), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
            macros::game_CaptureCutCommon(fighter);
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
}

#[acmd_script( agent = "chrom", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn chrom_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        sv_animcmd::frame(lua_state, 8.0);
            if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 4.0);
            }
        sv_animcmd::frame(lua_state, 9.0);
            if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
            }
        sv_animcmd::frame(lua_state, 10.0);
            if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 
            /*Z2 = 9.1 → 10.6*/ 3.0, 0.0, 7.25, 4.0, Some(0.0), Some(7.25), Some(10.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            
            macros::CATCH(fighter, 1, Hash40::new("top"), 
            /*Z2 = 10.4 → 12.0*/ 1.3, 0.0, 7.0, 2.7, Some(0.0), Some(7.0), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }//methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()();
            macros::game_CaptureCutCommon(fighter);
        sv_animcmd::wait(lua_state, 2.0);
            if macros::is_excute(fighter) {
                grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
            }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        chrom_sspecial, 
        chrom_uspecial, 
        chrom_uspecialair, 
        chrom_standinggrab, 
        chrom_dashgrab
    );
}
