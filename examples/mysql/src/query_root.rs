#[derive(Debug, seaography::macros::QueryRoot)]
#[seaography(entity = "crate::entities::staff")]
#[seaography(entity = "crate::entities::film_category")]
#[seaography(entity = "crate::entities::inventory")]
#[seaography(entity = "crate::entities::customer")]
#[seaography(entity = "crate::entities::city")]
#[seaography(entity = "crate::entities::film_actor")]
#[seaography(entity = "crate::entities::category")]
#[seaography(entity = "crate::entities::address")]
#[seaography(entity = "crate::entities::store")]
#[seaography(entity = "crate::entities::film")]
#[seaography(entity = "crate::entities::language")]
#[seaography(entity = "crate::entities::actor")]
#[seaography(entity = "crate::entities::payment")]
#[seaography(entity = "crate::entities::rental")]
#[seaography(entity = "crate::entities::film_text")]
#[seaography(entity = "crate::entities::country")]
pub struct QueryRoot;
