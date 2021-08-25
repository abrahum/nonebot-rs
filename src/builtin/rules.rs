use crate::config::BotConfig;
use crate::event::UserId;
use crate::matcher::Rule;

pub fn is_superuser<E>() -> Rule<E>
where
    E: UserId,
{
    let is_superuser = |event: &E, config: &BotConfig| -> bool {
        let user_id = event.ger_user_id();
        for superuser in &config.superusers {
            if &user_id == superuser {
                return true;
            }
        }
        false
    };
    is_superuser
}
