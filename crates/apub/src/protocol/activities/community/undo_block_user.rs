use crate::{
  objects::person::ApubPerson,
  protocol::{activities::community::block_user::BlockUserFromCommunity, Unparsed},
};
use activitystreams_kinds::activity::UndoType;
use lemmy_apub_lib::object_id::ObjectId;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoBlockUserFromCommunity {
  pub(crate) actor: ObjectId<ApubPerson>,
  #[serde(deserialize_with = "crate::deserialize_one_or_many")]
  pub(crate) to: Vec<Url>,
  pub(crate) object: BlockUserFromCommunity,
  #[serde(deserialize_with = "crate::deserialize_one_or_many")]
  pub(crate) cc: Vec<Url>,
  #[serde(rename = "type")]
  pub(crate) kind: UndoType,
  pub(crate) id: Url,
  #[serde(flatten)]
  pub(crate) unparsed: Unparsed,
}
