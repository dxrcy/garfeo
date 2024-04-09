use ibex::{routes, ssg};

#[macro_use]
mod views;

mod posts;
mod routes;
mod rss;

const URL_ROOT: &str = "/garfeo/";

fn main() {
    let posts = posts::parse_posts().expect("Failed to parse posts");

    let routes = routes! [
        // Normal pages
        (/)
            => routes::at_index(&posts),
        (/"plej-bonaj")
            => routes::at_favorites(&posts),
        (/"informejo")
            => routes::at_about(&posts),
        (/404)
            => routes::at_404(&posts),

        // List of posts
        (/"listo")
            => routes::at_list(&posts),
        // Grid of posts
        (/"krado")
            => routes::at_grid(&posts),

        // Posts (HTML)
        (/[post.get().index()])
            for post in posts
            => routes::at_post(post),
        (/"lasta")
            => routes::at_post(posts.first_ref()),

        // Posts (JSON)
        (/"index.json") | (/"posts.json")
            => ssg::raw(routes::json_index(&posts)),
        (/[post.get().index()]".json")
            for post in posts
            => ssg::raw(routes::json_post(post)),
        (/"latest.json")
            => ssg::raw(routes::json_post(posts.first_ref())),

        // RSS file
        (/"rss.xml")
            => ssg::raw(rss::generate_rss(posts)),
    ];

    ssg::quick_build(routes).expect("Failed to build");

    if ibex::is_local() {
        ssg::copy_or_symlink_dir("assets", "build/assets", true)
            .expect("Failed to copy or symlink assets");
    }

    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}
