use sea_orm::DbConn;

use app::DbPool;

#[derive(Debug, Clone, shaku::Component)]
#[shaku(interface = DbPool)]
pub struct Pool {
    db_conn: DbConn,
}

impl DbPool for Pool {
    fn get(&self) -> &DbConn {
        &self.db_conn
    }
}
