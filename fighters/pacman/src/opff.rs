// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

//PM-like neutral-b canceling
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

//Pac-Man Bonus Fruit Toss Airdodge Cancel
unsafe fn fruit_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_SHOOT {
        if frame > 8.0 {
            boma.check_airdodge_cancel();
        }
    }
}

//Side B Run Cancel
unsafe fn pacrun(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH 
    && situation_kind == *SITUATION_KIND_GROUND 
    && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
        StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_RUN, true);
    }
}

//Dair bounce
unsafe fn dair_bounce(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    let boma = fighter.boma();
    let bounce_num = VarModule::get_int(boma.object(), vars::pacman::instance::DAIR_BOUNCE);
    let bounce_lim = 1;
    if boma.is_motion(Hash40::new("attack_air_lw")) 
    && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
    && !fighter.is_in_hitlag() {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        //Limit amount of recoil momentum frames from bounce
        if bounce_num < bounce_lim { 
            KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::add_speed(boma, &Vector3f::new(0.0, 1.5, 0.0));
            VarModule::set_int(boma.object(), vars::pacman::instance::DAIR_BOUNCE, bounce_num + 1);
        }
    }
    //Reset on touching ground or trampoline
    if fighter.is_situation(*SITUATION_KIND_GROUND) 
    || fighter.is_status(*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP) {
        VarModule::set_int(boma.object(), vars::pacman::instance::DAIR_BOUNCE, 0);
    }
}


//Jump out of trampoline, works around Pac-Man's unique SpecialHi fall state flag
unsafe fn tramp_jump(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    //Trampoline resets jumps so store prior to jump check
    if fighter.get_num_used_jumps() == 2 {
        VarModule::on_flag(fighter.object(), vars::pacman::instance::IS_USE_DJ);
    }
    //Set tramp flag
    if fighter.is_status_one_of(&[
    *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP,
    *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_END,
    *FIGHTER_STATUS_KIND_SPECIAL_HI,
    ]) {
        VarModule::on_flag(fighter.object(), vars::pacman::instance::IS_USE_TRAMP);      
    } else {
        //Reset flags
        if fighter.is_status(*FIGHTER_STATUS_KIND_LANDING) {
            VarModule::off_flag(fighter.object(), vars::pacman::instance::IS_USE_TRAMP);
            VarModule::off_flag(fighter.object(), vars::pacman::instance::IS_USE_DJ);
        }
    }
    //Conditions for jumping after trampoline
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL) {
        if fighter.is_cat_flag(Cat1::JumpButton | Cat1::Jump)
        && !VarModule::is_flag(fighter.object(), vars::pacman::instance::IS_USE_DJ)
        && CancelModule::is_enable_cancel(fighter.module_accessor) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
        //After performing any action (except airdodge), don't revert back to fall state
        } else if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
        ]) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FALL);
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    nspecial_cancels(boma, status_kind, situation_kind);
    fruit_ac(boma, status_kind, frame);
    //pacrun(boma, status_kind, situation_kind, cat[0], stick_y);
    dair_bounce(fighter);
    tramp_jump(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_PACMAN )]
pub fn pacman_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pacman_frame(fighter);
        println!("dj: {}\ntramp: {}", VarModule::is_flag(fighter.object(), vars::pacman::instance::IS_USE_DJ), VarModule::is_flag(fighter.object(), vars::pacman::instance::IS_USE_TRAMP));
    }
}

pub unsafe fn pacman_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}