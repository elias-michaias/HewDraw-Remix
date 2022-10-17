
use super::*;

#[acmd_script( agent = "pacman", script = "game_specialnshoot" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            let charge_level = WorkModule::get_int(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
            if charge_level <= 0 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANCHERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 2 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANSTRAWBERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 4 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANORANGE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 6 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANAPPLE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 8 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANMELON), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 10 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBOSS), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 12 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBELL), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 14 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANKEY), 0, 0, false, false);   
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            //WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_FLAG_THROW);
        }
        
    }
    
}

#[acmd_script( agent = "pacman", script = "game_specialairnshoot" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_air_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            let charge_level = WorkModule::get_int(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
            if charge_level <= 0 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANCHERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 2 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANSTRAWBERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 4 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANORANGE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 6 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANAPPLE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 8 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANMELON), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 10 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBOSS), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 12 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBELL), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 14 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANKEY), 0, 0, false, false);   
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            //WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_FLAG_THROW);
        }
        
    }
    
}


#[acmd_script( agent = "pacman", script = "game_speciallwfailure" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_lw_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.4);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 270, 70, 0, 20, 10.0, 0.0, 3.5, 0.0, None, None, None, 1.6, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pacman", script = "game_specialairlwfailure" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_air_lw_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.4);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 270, 70, 0, 20, 10.0, 0.0, 3.5, 0.0, None, None, None, 1.6, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pacman", scripts = ["game_specialsstart" , "game_specialsloop"] , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy_all(boma);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}


#[acmd_script( agent = "pacman_firehydrant", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_firehydrant_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot2"), 11.0, 45, 90, 0, 30, 5.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "pacman_firehydrant", script = "game_fall" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_firehydrant_fall_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 70, 80, 0, 60, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
}

#[acmd_script( agent = "pacman", script = "game_specialairhiloop" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_air_hi_loop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();    
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 5.0, 92, 90, 0, 30, 5.0, -0.5, 2.0, 0.0, Some(0.5), Some(2.0), None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 6.0, 86, 95, 0, 40, 4.4, -0.5, 2.0, 0.0, Some(0.5), Some(2.0), None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL);
        ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 7.0, 60, 100, 0, 40, 3.8, -0.5, 2.0, 0.0, Some(0.5), Some(2.0), None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}



pub fn install() {
    install_acmd_scripts!(
        //pacman_special_n_shoot_game,
        //pacman_special_air_n_shoot_game,
        pacman_special_lw_failure_game,
        pacman_special_air_lw_failure_game,
        pacman_special_s_start_game,
        pacman_special_air_hi_loop,

        //hydrant
        pacman_firehydrant_fly_game,
        pacman_firehydrant_fall_game,
    );
}

