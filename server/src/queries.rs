use diesel::prelude::*;

use schema;
use schema::DbConn;
use schema::Entry;
use schema::Meet;
use schema::Lifter;


/// Count the number of rows in the "Entries" table.
pub fn count_entries(conn: &DbConn) -> Option<i64> {
    schema::entries::table
        .count()
        .get_result(&**conn)
        .ok()
}


/// Count the number of rows in the "Meets" table.
pub fn count_meets(conn: &DbConn) -> Option<i64> {
    schema::meets::table
        .count()
        .get_result(&**conn)
        .ok()
}


/// Look up a Meet by its human-readable MeetPath.
pub fn get_meet_by_meetpath(meetpath: &str, conn: &DbConn) -> Option<Meet> {
    schema::meets::table
        .filter(schema::meets::MeetPath.eq(meetpath))
        .first::<Meet>(&**conn)
        .ok()
}

/// Look up all the Entries that correspond to a given MeetID.
pub fn get_entries_by_meetid(meetid: i32, conn: &DbConn) -> Option<Vec<Entry>> {
    schema::entries::table
        .filter(schema::entries::MeetID.eq(meetid))
        .load::<Entry>(&**conn)
        .ok()
}

/// Look up a Lifter by Username.
pub fn get_lifter_by_username(username: &str, conn: &DbConn) -> Option<Lifter> {
    schema::lifters::table
        .filter(schema::lifters::Username.eq(username))
        .first::<Lifter>(&**conn)
        .ok()
}

/// Look up all Entries for a given LifterID.
pub fn get_entries_by_lifterid(lifterid: i32, conn: &DbConn) -> Option<Vec<(Entry, Meet)>> {
    schema::entries::table
        .filter(schema::entries::LifterID.eq(lifterid))
        .inner_join(schema::meets::table)
        .order(schema::meets::MeetDate.desc())
        .load(&**conn)
        .ok()
}


#[cfg(test)]
mod test {
    use super::*;
    use schema;
    use schema::DbConn;

    fn db() -> DbConn {
        DbConn(schema::init_pool("../build/openpowerlifting.sqlite3").get().unwrap())
    }

    #[test]
    fn test_count_entries() {
        let count = count_entries(&db()).unwrap();
        assert!(count > 300_000);
    }

    #[test]
    fn test_count_meets() {
        let count = count_meets(&db()).unwrap();
        assert!(count > 7_000);
    }

    #[test]
    fn test_get_meet_by_meetpath() {
        let meet = get_meet_by_meetpath("uspa/0485", &db()).unwrap();
        assert_eq!(meet.name, "Boss of Bosses 2");
    }

    #[test]
    fn test_get_entries_by_meetid() {
        let conn = db();

        // Because the MeetID changes on every database recompilation,
        // key the test data off the MeetPath.
        let meet = get_meet_by_meetpath("uspa/0485", &conn).unwrap();
        let entries = get_entries_by_meetid(meet.id, &conn).unwrap();

        assert_eq!(entries.len(), 155);
        for entry in entries {
            assert_eq!(entry.meet_id, meet.id);
        }
    }

    #[test]
    fn test_get_lifter_by_username() {
        let conn = db();
        let lifter = get_lifter_by_username("seanstangl", &conn).unwrap();
        assert_eq!(lifter.id, 0);
        assert_eq!(lifter.name, "Sean Stangl");
        assert_eq!(lifter.instagram, Some("ferruix".into()));
    }

    #[test]
    fn test_no_instagram_is_null() {
        let conn = db();
        // Select a lifter who is definitely not going to have an Instagram.
        let lifter = get_lifter_by_username("lamargant", &conn).unwrap();
        assert_eq!(lifter.name, "Lamar Gant");
        assert_eq!(lifter.instagram, None);
    }
}
