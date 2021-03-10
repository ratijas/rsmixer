use super::common::*;

use crate::ui::Scrollable;

pub async fn action_handler(msg: &Action, state: &mut RSState, ctx: &Ctx) {
    match msg.clone() {
        Action::MoveUp(how_much) => {
            if let UIMode::MoveEntry(_, _) = state.ui_mode {
                if state.page_entries.entries.len() < 2 {
                    return;
                }
                let l = (state.page_entries.len() - 1) as i32;
                let selected = (state.page_entries.selected() - 1) as i32;

                let mut j = selected - how_much as i32;

                if j < 0 {
                    j = j.abs() % l;
                    j = l - j;
                }

                if j >= selected {
                    j += 1;
                }

                let entry_ident = state.page_entries.get_selected().unwrap();
                let new_parent = state.page_entries.get(j as usize).unwrap();
                state.ui_mode = UIMode::MoveEntry(entry_ident, new_parent);

                ctx.send_to("event_loop", Action::Redraw);
            }
        }
        Action::MoveDown(how_much) => {
            if let UIMode::MoveEntry(_, _) = state.ui_mode {
                if state.page_entries.entries.len() < 2 {
                    return;
                }
                let l = state.page_entries.len() - 1;
                let selected = state.page_entries.selected() - 1;

                let mut j = (selected + how_much as usize) % l;

                if j >= selected {
                    j += 1;
                }

                let entry_ident = state.page_entries.get_selected().unwrap();
                let new_parent = state.page_entries.get(j as usize).unwrap();
                state.ui_mode = UIMode::MoveEntry(entry_ident, new_parent);

                ctx.send_to("event_loop", Action::Redraw);
            }
        }
        Action::Confirm => match state.ui_mode {
            UIMode::MoveEntry(ident, parent) => {
                state.change_ui_mode(UIMode::Normal);
                ctx.send_to("pulseaudio", Action::MoveEntryToParent(ident, parent));
            }
            _ => {
                return;
            }
        },
        _ => {
            return;
        }
    }
}