use crate::{
    error::ContractError,
    msg::NodeEditMsg,
    state::storage::{NODE_ID_2_ATTACHMENT, NODE_ID_2_BODY, NODE_ID_2_TITLE},
    util::{load_node_metadata, process_tags_and_handles},
};
use cosmwasm_std::{attr, Order, Response};

use super::Context;

pub fn exec_edit_node(
    ctx: Context,
    msg: NodeEditMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let metadata = load_node_metadata(deps.storage, msg.id, true)?.unwrap();

    if let Some(new_body) = &msg.body {
        process_tags_and_handles(deps.storage, msg.id, msg.tags, msg.handles, true)?;
        // TODO: validate new body
        NODE_ID_2_BODY.save(deps.storage, msg.id, new_body)?;
        if msg.title.is_some() {
            if metadata.parent_id.is_some() {
                return Err(ContractError::ValidationError {
                    reason: "Only the root node has a title".to_owned(),
                });
            } else {
                // TODO: validate new title
                let title = msg.title.unwrap();
                NODE_ID_2_TITLE.save(deps.storage, msg.id, &title)?;
            }
        }
    }

    if let Some(new_attachments) = &msg.attachments {
        // TODO: validate attachments
        // Remove old attachements
        for i in NODE_ID_2_ATTACHMENT
            .prefix(msg.id)
            .keys(deps.storage, None, None, Order::Ascending)
            .map(|r| r.unwrap())
            .collect::<Vec<u8>>()
        {
            NODE_ID_2_ATTACHMENT.remove(deps.storage, (msg.id, i as u8));
        }
        // Save new attachements
        for (i, attachment) in new_attachments.iter().enumerate() {
            NODE_ID_2_ATTACHMENT.save(deps.storage, (msg.id, i as u8), attachment)?;
        }
    }

    Ok(Response::new().add_attributes(vec![attr("action", "edit")]))
}
