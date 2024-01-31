// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
use utils::consts::vars::gekkouga::instance::FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_LW_LOG_ID;

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

//Down special creates a visual blocking cloud and spawns a substitute doll with a hop backwards
unsafe fn substitute(fighter: &mut L2CFighterCommon) {
    let mut lr = 1.0;
    let timer = VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_TIMER);
    let life = VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_LIFE);
    let boma = fighter.boma();
    let kind = boma.kind();
    let x = PostureModule::pos_x(boma);
    let y = PostureModule::pos_y(boma);

    let doll_id = WorkModule::get_int64(fighter.module_accessor,vars::gekkouga::instance::FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_LW_DOLL_ID) as u32;
    let doll_boma = sv_battle_object::module_accessor(doll_id);
    let doll_kind = sv_battle_object::kind(doll_id);

    let log_id = WorkModule::get_int64(fighter.module_accessor,vars::gekkouga::instance::FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_LW_LOG_ID) as u32;
    let log_boma = sv_battle_object::module_accessor(log_id);
    let log_kind = sv_battle_object::kind(log_id);

    //Timing checks on various things
    if timer < 0 {
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_TIMER, 1);
    }
    if timer > 0 {
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_TIMER, timer - 1);
    }
    if timer == 1 {
        gimmick_flash(fighter);
    }
    if life > 0 {
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_LIFE, life - 1);
    } 
    if life == 0 {
        if doll_kind == *ITEM_KIND_DOLL {
            StatusModule::change_status_request_from_script(doll_boma, *ITEM_STATUS_KIND_LOST, true);
        } else if log_kind == *ITEM_KIND_LOG {
            StatusModule::change_status_request_from_script(log_boma, *ITEM_STATUS_KIND_LOST, true);
            println!("LOL");
        }
    }
    if smoke > 0 {
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SMOKESCREEN_LIFE, smoke - 1);
    }

    //Startup on press
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_HIT, true);
        lr = PostureModule::lr(fighter.boma());
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_TIMER, 480);
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_LIFE, 360);
        VarModule::on_flag(fighter.object(), vars::gekkouga::instance::IS_MANUAL_USAGE);
        FT_ADD_DAMAGE(fighter, 5.0);
    }

    //Transition to spawn
    if fighter.is_status(*FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_HIT)
    && fighter.motion_frame() > 5.0
    && VarModule::is_flag(fighter.object(), vars::gekkouga::instance::IS_MANUAL_USAGE) {
        //Falling hitbox
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if doll_kind == *ITEM_KIND_DOLL {
                MotionModule::change_motion(doll_boma, Hash40::new("throw"), 0.0, 1.0, false, 0.0, false, false);
            } else if log_kind == *ITEM_KIND_LOG {
                MotionModule::change_motion(log_boma, Hash40::new("throw"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        //Greninja state
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(-5.5 * lr, 1.0, 0.0)); 
        //Refresh variables
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT, 0);
        VarModule::off_flag(fighter.object(), vars::gekkouga::instance::IS_MANUAL_USAGE);
        VarModule::on_flag(fighter.object(), vars::gekkouga::instance::IS_SUBSTITUTE_SPECIAL);
    }
}

//Logic for substitute doll/log
pub unsafe fn substitute_doll(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    let boma = fighter.boma();
    let kind = fighter.kind();
    let x = PostureModule::pos_x(boma);
    let y = PostureModule::pos_y(boma);
    let lr = PostureModule::lr(boma);
    let timer = VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_TIMER);
    let count = VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT);

    //Get substitute doll
    let doll_id = WorkModule::get_int64(fighter.module_accessor,vars::gekkouga::instance::FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_LW_DOLL_ID) as u32;
    let doll_kind = sv_battle_object::kind(doll_id);

    //Get substitute log
    let log_id = WorkModule::get_int64(fighter.module_accessor,vars::gekkouga::instance::FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_LW_LOG_ID) as u32;
    let log_kind = sv_battle_object::kind(log_id);

    //Check which one it is
    let mut doll_boma = sv_battle_object::module_accessor(doll_id);
    if log_kind == *ITEM_KIND_LOG {
        doll_boma = sv_battle_object::module_accessor(log_id);
    } else if doll_kind != *ITEM_KIND_DOLL {
        return;
    }

    //Check which Greninja owns it
    if LinkModule::get_parent_id(doll_boma, *ITEM_LINK_NO_CREATEOWNER, true) as u32 != fighter.battle_object_id {
        return
    }

    let doll_x = PostureModule::pos_x(doll_boma);
    let doll_y = PostureModule::pos_y(doll_boma);

    if VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_LIFE) == 0 {
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT, 0);
        VarModule::off_flag(fighter.object(), vars::gekkouga::instance::IS_SUBSTITUTE_SPECIAL);
    }

    let delta_x = lr * (doll_x - x);
    let delta_y = doll_y - y;

    //Count until it's been hit twice
    if StopModule::is_damage(doll_boma) {
        VarModule::on_flag(fighter.object(), vars::gekkouga::instance::IS_SUBSTITUTE_HIT);
        MotionModule::change_motion(doll_boma, Hash40::new("throw"), 0.0, 1.0, false, 0.0, false, false);
    } else if VarModule::is_flag(fighter.object(), vars::gekkouga::instance::IS_SUBSTITUTE_HIT) {
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT, count + 1);
        VarModule::off_flag(fighter.object(), vars::gekkouga::instance::IS_SUBSTITUTE_HIT);
    }

    //When hit twice, substitute doll vanishes
    if VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT) > 1 
    && VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT) < 69 {
        //Step 1: Start poof script and prevent doll from dying during script
        HitModule::set_whole(doll_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
        StatusModule::set_situation_kind(doll_boma, app::SituationKind(*SITUATION_KIND_AIR), true);
        MotionModule::change_motion(doll_boma, Hash40::new("poof"), 0.0, 1.0, false, 0.0, false, false);
        EFFECT(fighter, Hash40::new("gekkouga_kageuchi_warp_start"), Hash40::new("top"), delta_x, delta_y - 6.5, 3.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        PLAY_SE_REMAIN(fighter, Hash40::new("se_gekkouga_special_s02"));
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT, 69);
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_LIFE, VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_LIFE) + 2);
    }
    if VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT) >= 420 {
        //Step 3: Delete the doll and reset counter
        StatusModule::change_status_request_from_script(doll_boma, *ITEM_STATUS_KIND_LOST, true);
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_LIFE, 0);
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT, 0);
    }
    if VarModule::get_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT) >= 69 {
        //Step 2: Give the doll a frame for the script to work properly
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_HIT_COUNT, 420);
    }

    //Taunt while doll is out to consume doll and gain half of health and time cost back
    if fighter.is_status(*FIGHTER_STATUS_KIND_APPEAL) 
    && fighter.motion_frame() > 25.0
    && VarModule::is_flag(fighter.object(), vars::gekkouga::instance::IS_SUBSTITUTE_SPECIAL) {
        //Doll FX
        PLAY_SE_REMAIN(fighter, Hash40::new("se_gekkouga_attack_water"));
        EFFECT(fighter, Hash40::new("gekkouga_water_impact"), Hash40::new("top"), delta_x, delta_y, 17.0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        //Greninja FX
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x91AAE256A), Hash40::new("top"), -7, 2.0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 1.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        EFFECT_FOLLOW(fighter, Hash40::new("gekkouga_pump_splash"), Hash40::new("rot"), 0, -4, 0, 270, 0, 0, 0.7, true);
        EFFECT(fighter, Hash40::new("gekkouga_pump_hit"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.35);
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
        FT_ADD_DAMAGE(fighter, -2.5);
        StatusModule::change_status_request_from_script(doll_boma, *ITEM_STATUS_KIND_LOST, true);
        VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_LIFE, 0);
        if timer <= 240 {
            VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_TIMER, 1);
        } else {
            VarModule::set_int(fighter.object(), vars::gekkouga::instance::SUBSTITUTE_TIMER, timer - 240);
        }
        VarModule::off_flag(fighter.object(), vars::gekkouga::instance::IS_SUBSTITUTE_SPECIAL);
    }   
}
 
unsafe fn max_water_shuriken_dc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT {
        if frame > 12.0 {
            boma.check_dash_cancel();
        }
    }
}

// Greninja Shadow Sneak Smash Attack Cancel
unsafe fn shadow_sneak_smash_attack_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK {
        if boma.status_frame() < 6 {
            if situation_kind == *SITUATION_KIND_GROUND {
                if boma.is_cat_flag(Cat1::AttackS4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                }
                if boma.is_cat_flag(Cat1::AttackHi4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                }
                if boma.is_cat_flag(Cat1::AttackLw4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                }
            }
        }
    }
}


unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_SHOT,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_START,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_WALL_DAMAGE,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_ATTACK,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_BOUND
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    max_water_shuriken_dc(boma, status_kind, situation_kind, cat[0], frame);
    shadow_sneak_smash_attack_cancel(boma, status_kind, situation_kind, cat[0], frame);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_GEKKOUGA )]
pub fn gekkouga_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        substitute(fighter);
        substitute_doll(fighter);
		gekkouga_frame(fighter);
    }
}

pub unsafe fn gekkouga_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}