use serde::Deserialize;

fn parse_direction_to_sea_orm_order(
    direction: &str,
) -> Result<sea_orm::Order, serde::de::Unexpected> {
    match direction {
        "asc" => Ok(sea_orm::Order::Asc),
        "desc" => Ok(sea_orm::Order::Desc),
        _ => Err(serde::de::Unexpected::Str(direction)),
    }
}

#[derive(Debug)]
pub struct SortTuple(pub String, pub sea_orm::Order);

struct SortTupleVisitor;

impl<'de> serde::de::Visitor<'de> for SortTupleVisitor {
    type Value = SortTuple;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string in format: 'column-direction', e.g., 'title-asc'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let parts = v.split("-").collect::<Vec<_>>();

        if parts.len() != 2 {
            // Error out
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            ))
        } else {
            let column = parts[0].to_string();
            let direction = parts[1].to_string();

            if column.is_empty() {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(v),
                    &self,
                ));
            }

            let direction = parse_direction_to_sea_orm_order(&direction);
            if direction.is_err() {
                return Err(serde::de::Error::invalid_value(
                    direction.err().unwrap(),
                    &self,
                ));
            }

            Ok(SortTuple(column, direction.unwrap()))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
}

impl<'de> Deserialize<'de> for SortTuple {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(SortTupleVisitor)
    }
}

fn get_default_sort_options() -> SortTuple {
    SortTuple("rating".to_string(), sea_orm::Order::Desc)
}

fn get_default_limit() -> u64 {
    10
}

fn get_default_page() -> u64 {
    1
}

#[derive(Debug, Deserialize)]
pub struct GetMoviesQueryParams {
    #[serde(default = "get_default_limit")]
    pub limit: u64,
    #[serde(default = "get_default_page")]
    pub page: u64,
    #[serde(rename = "sort", default = "get_default_sort_options")]
    pub sort_options: SortTuple,
}
