use crate::gql::root::Ctx;
use crate::models::user::{NewUser, UpdateUser, Users};
use chrono::NaiveDate;
use diesel::prelude::*;
use juniper::FieldResult;
// use crate::crud::base::CO;


// impl CO for Users{
//     type All=Vec<Users>;
//     type Get=FieldResult<Users>;

//     fn all(&self,ctx: &Ctx) -> Vec<Users> {
//         use crate::schema::users::dsl::*;
//         let connection = ctx.db.get().unwrap();
//         users
//             .limit(100)
//             .load::<Self>(&connection)
//             .expect("Error loading users")
//     }
    
//     fn by_id(&self,ctx: &Ctx, id: String) -> FieldResult<Users> {
//         use crate::schema::users::dsl::*;
//         let connection = ctx.db.get().unwrap();
//         let result_user = users.filter(user_id.eq(id)).first::<Self>(&connection)?;
//         Ok(result_user)
//     }
    
// }


pub fn allusers(ctx: &Ctx) -> Vec<Users> {
    use crate::schema::users::dsl::*;
    let connection = ctx.db.get().unwrap();
    users
        .limit(100)
        .load::<Users>(&connection)
        .expect("Error loading users")
}

pub fn user(ctx: &Ctx, uid: String) -> FieldResult<Users> {
    use crate::schema::users::dsl::*;
    let connection = ctx.db.get().unwrap();
    let result_user = users.filter(user_id.eq(uid)).first::<Users>(&connection)?;
    Ok(result_user)
}

pub fn create_user(new_user: NewUser, ctx: &Ctx) -> FieldResult<Vec<Users>> {
    use crate::schema::users::dsl::*;
    let connection = ctx.db.get().unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    let user = Users {
        user_id: id,
        first_name: new_user.first_name,
        middle_name: new_user.middle_name,
        last_name: new_user.last_name,
        address_id: new_user.address_id,
        email: new_user.email.to_lowercase(),
        phone: new_user.phone,
        password_hash: new_user.password,
        registered_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        last_login: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        rating: new_user.rating,
        profile: new_user.profile,
    };
    let res = diesel::insert_into(users)
        .values(user)
        .get_results::<Users>(&connection);
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
    }
}

pub fn update(ctx: &Ctx, id: String, updated_user: UpdateUser) -> FieldResult<Users> {
    use crate::schema::users::dsl::*;
    let connection = ctx.db.get().unwrap();
    let result_user = diesel::update(users)
        .filter(user_id.eq(id))
        .set(updated_user)
        .get_result::<Users>(&connection)?;
    Ok(result_user)
}

pub fn delete(ctx: &Ctx, id: String) -> FieldResult<Users> {
    use crate::schema::users::dsl::*;
    let connection = ctx.db.get().unwrap();
    let deleted_user =
        diesel::delete(users.filter(user_id.eq(id))).get_result::<Users>(&connection)?;
    Ok(deleted_user)
}
