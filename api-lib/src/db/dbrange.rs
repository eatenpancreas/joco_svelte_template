



#[macro_export]
macro_rules! query_db_range {
    ($typ: ty, $range:ident, $executor:ident, $selector:tt, $search_on:tt, $precision: stmt) => {
        {
            let mut order_by = match $range.order_by {
                Some(mut x) => {
                    x.to_string();
                    x.push_str(", ");
                    x
                },
                None => { String::new() }
            };
            
            let limit = $range.limit.unwrap_or(10) as i64;
            let offset = $range.offset.unwrap_or(0) as i64;
            
            match $range.search {
                Some(src) => {
                    if let Direction::Desc = $range.direction.unwrap_or(Direction::Asc) {
                        sqlx::query_as!(Self, $selector + " WHERE similarity(" + $search_on + ", $1) >= $5 ORDER BY $2, similarity(" + $search_on + ", $1) DESC LIMIT $3 OFFSET $4", 
                          src, order_by, limit, offset,
                          $precision
                        ).fetch_all($executor).await.ok()
                    } else {
                        sqlx::query_as!(Self, $selector + " WHERE similarity(" + $search_on + ", $1) >= $5 ORDER BY $2, similarity(" + $search_on + ", $1) ASC LIMIT $3 OFFSET $4", 
                          src, order_by, limit, offset,
                          $precision 
                        ).fetch_all($executor).await.ok()
                    }
                }
                None => {
                    if let Direction::Desc = $range.direction.unwrap_or(Direction::Asc) {
                        sqlx::query_as!(Self, $selector + " ORDER BY $1, similarity(" + $search_on + ", $1) DESC LIMIT $2 OFFSET $3", 
                            order_by, limit, offset
                        ).fetch_all($executor).await.ok()
                    } else {
                        sqlx::query_as!(Self, $selector + " ORDER BY $1, similarity(" + $search_on + ", $1) ASC LIMIT $2 OFFSET $3", 
                            order_by, limit, offset
                        ).fetch_all($executor).await.ok()
                    }
                }
            }
        }
    }
}