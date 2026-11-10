pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use self::models::{NewPost, Post};
use self::schema::posts::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut conn = establish_connection();

    let new_post = NewPost {
        title: "สวัสดี Rust และ Diesel",
        body: "นี้คือบทความแรกที่บันทึกลง SQLite",
    };

    let inserted_post: Post = diesel::insert_into(posts)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post");

    println!("สร้างสำเร็จ: {:?}", inserted_post);

    let updated_post: Post = diesel::update(posts.find(inserted_post.id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(&mut conn)
        .expect("Error updating post");
    println!("อัพเดตสำเร็จ: {:?}", updated_post);

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    println!("\nรายการบทความทั้งหมด ({} รายการ)", results.len());
    for post in results {
        println!("- [{}] {}: {}", post.id, post.title, post.body);
    }

    let num_deleted = diesel::delete(posts.filter(id.eq(inserted_post.id)))
        .execute(&mut conn)
        .expect("Error deleting post");
    println!("\nลบข้อมูลไปทั้งหมด {} รายการ", num_deleted);
}
