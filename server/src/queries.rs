
#[cfg(test)]
mod test {
    use diesel::prelude::*;

    use schema;
    use schema::Meet;
    use schema::DbConn;

    fn get_test_dbconn() -> DbConn {
        DbConn(schema::init_pool().get().unwrap())
    }

    #[test]
    fn database_works() {
        // Early canary to see if the database works at all.
        let conn = get_test_dbconn();
        let bob2_path = "uspa/0485";
        let bob2_meetname = "Boss of Bosses 2";

        let meet =
            schema::meets::table.filter(schema::meets::MeetPath.eq(bob2_path))
            .first::<Meet>(&*conn)
            .unwrap();

        assert_eq!(meet.name, bob2_meetname);
    }
}
