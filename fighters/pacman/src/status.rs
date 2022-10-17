use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        specialhiend_main,
    );
}

pub unsafe extern "C" fn specialhiend_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) || !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if fighter.is_prev_situation(*SITUATION_KIND_GROUND) || !fighter.is_situation(*SITUATION_KIND_AIR) {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi_end"), -1.0, 1.0, 0.0, false, false);
        } else if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_end"), -1.0, 1.0, 0.0, false, false);
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0, 0, 0, 0, 0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0, 0);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            lua_args!(fighter, 0.0);
            app::sv_kinetic_energy::controller_set_accel_x_add(fighter.lua_state_agent);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0, 0);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
        //if MotionModule::is_end(fighter.module_accessor) {
        if fighter.motion_frame() >= 4.0 {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
        return 0.into()
    } else {
        return 1.into()
    }
}

#[status_script(agent = "pacman", status = FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe extern "C" fn specialhiend_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_rot(fighter.module_accessor, &smash::phx::Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.main_shift(specialhiend_main_loop)
}