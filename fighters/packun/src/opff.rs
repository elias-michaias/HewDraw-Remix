// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn piranhacopter_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, false);
        }
    }
}

// Spike Mesh Visibility Test
unsafe fn spike_head_mesh_test(boma: &mut BattleObjectModuleAccessor) {
    // HeadA is the normal head
	// HeadB is the poison head
	// HeadS is the spike head
	ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), false);
	ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), false);
	ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), true);
}

unsafe fn sspecial_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE) == *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_NONE, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}


pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    piranhacopter_cancel(boma, status_kind, cat[0]);
	//spike_head_mesh_test(boma);
    sspecial_cancel(boma, status_kind, situation_kind);
}

#[utils::macros::opff(FIGHTER_KIND_PACKUN )]
pub fn packun_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		packun_frame(fighter)
    }
}

pub unsafe fn packun_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}