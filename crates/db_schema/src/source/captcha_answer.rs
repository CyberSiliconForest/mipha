#[cfg(feature = "full")]
use crate::schema::captcha_answer;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = captcha_answer))]
pub struct CaptchaAnswer {
  pub uuid: String,
  pub answer: String,
  pub expires: chrono::NaiveDateTime,
}
