use crate::{
    domain::prelude::{Token, UserAgent, UserId},
    generate_entity,
};

generate_entity!(CreateDeviceCommand {
    token: Token,
    user_agent: UserAgent,
    user_id: UserId
});

generate_entity!(InvalidateDeviceCommand { token: Token });
